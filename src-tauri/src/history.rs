use crate::config::ConfigManager;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

const RETENTION_SECONDS: u64 = 90 * 86400; // 90 days retention

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BreakRecord {
    pub timestamp: u64,
    pub date: String,
    pub action: String, // "completed", "snoozed", "dismissed"
    pub duration_seconds: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DailyStat {
    pub day: String,  // "Mon", "Tue", etc.
    pub date: String, // "YYYY-MM-DD"
    pub count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BreakStats {
    pub breaks_today: u32,
    pub breaks_this_week: u32,
    pub daily_average: f32,
    pub current_streak: u32,
    pub best_streak: u32,
    pub last_7_days: Vec<DailyStat>,
}

pub struct HistoryManager {
    history_path: PathBuf,
    records: Arc<RwLock<Vec<BreakRecord>>>,
}

impl HistoryManager {
    pub fn new() -> Self {
        let history_path = ConfigManager::resolve_data_dir().join("history.json");
        let records = Self::load_records(&history_path);
        Self {
            history_path,
            records: Arc::new(RwLock::new(records)),
        }
    }

    #[cfg(test)]
    pub fn new_with_path(path: PathBuf) -> Self {
        let records = Self::load_records(&path);
        Self {
            history_path: path,
            records: Arc::new(RwLock::new(records)),
        }
    }

    pub fn record_break(&self, action: &str, duration_seconds: u32) -> Result<(), String> {
        let now_secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let (date_str, _) = timestamp_to_date(now_secs);
        let record = BreakRecord {
            timestamp: now_secs,
            date: date_str,
            action: action.to_string(),
            duration_seconds,
        };

        if let Ok(mut records) = self.records.write() {
            records.push(record);

            // Prune entries older than 90 days
            let cutoff = now_secs.saturating_sub(RETENTION_SECONDS);
            records.retain(|r| r.timestamp >= cutoff);

            Self::save_records(&self.history_path, &records)?;
        }

        Ok(())
    }

    pub fn get_stats(&self) -> BreakStats {
        let now_secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let records = self.records.read().map(|r| r.clone()).unwrap_or_default();
        Self::compute_stats(&records, now_secs)
    }

    fn compute_stats(records: &[BreakRecord], now_secs: u64) -> BreakStats {
        let (today_str, _) = timestamp_to_date(now_secs);
        let today_days = (now_secs / 86400) as i64;
        let week_start_days = today_days - 6;

        let mut breaks_today = 0;
        let mut breaks_this_week = 0;
        let mut best_streak = 0;
        let mut running_streak = 0;

        for record in records {
            let is_positive_break = record.action == "completed" || record.action == "dismissed";
            if is_positive_break {
                if record.date == today_str {
                    breaks_today += 1;
                }
                let record_days = (record.timestamp / 86400) as i64;
                if record_days >= week_start_days && record_days <= today_days {
                    breaks_this_week += 1;
                }
                running_streak += 1;
                if running_streak > best_streak {
                    best_streak = running_streak;
                }
            } else if record.action == "snoozed" {
                // Snooze does not reset streak, but doesn't increment
            }
        }
        let current_streak = running_streak;

        // Daily average over active recorded days (or 7 days if history exists)
        let unique_days = records
            .iter()
            .filter(|r| r.action == "completed" || r.action == "dismissed")
            .map(|r| &r.date)
            .collect::<std::collections::HashSet<_>>()
            .len();

        let total_breaks: u32 = records
            .iter()
            .filter(|r| r.action == "completed" || r.action == "dismissed")
            .count() as u32;

        let daily_average = if unique_days > 0 {
            (total_breaks as f32 / unique_days as f32 * 10.0).round() / 10.0
        } else {
            0.0
        };

        // Last 7 days breakdown
        let mut last_7_days = Vec::with_capacity(7);
        for i in (0..7).rev() {
            let target_day_secs = (today_days - i) as u64 * 86400 + 43200; // mid-day
            let (target_date, target_day_name) = timestamp_to_date(target_day_secs);

            let count = records
                .iter()
                .filter(|r| {
                    r.date == target_date
                        && (r.action == "completed" || r.action == "dismissed")
                })
                .count() as u32;

            last_7_days.push(DailyStat {
                day: target_day_name,
                date: target_date,
                count,
            });
        }

        BreakStats {
            breaks_today,
            breaks_this_week,
            daily_average,
            current_streak,
            best_streak,
            last_7_days,
        }
    }

    fn load_records(path: &PathBuf) -> Vec<BreakRecord> {
        if path.exists() {
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(records) = serde_json::from_str::<Vec<BreakRecord>>(&content) {
                    return records;
                }
            }
        }
        Vec::new()
    }

    fn save_records(path: &PathBuf, records: &[BreakRecord]) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                let _ = fs::create_dir_all(parent);
            }
        }
        let json = serde_json::to_string_pretty(records).map_err(|e| e.to_string())?;
        fs::write(path, json).map_err(|e| e.to_string())?;
        Ok(())
    }
}

/// Convert unix timestamp in seconds to ("YYYY-MM-DD", "DayOfWeek")
pub fn timestamp_to_date(ts: u64) -> (String, String) {
    let days_since_epoch = (ts / 86400) as i64;
    let (year, month, day) = days_to_ymd(days_since_epoch);

    // 1970-01-01 was Thursday (index 4 in [Sun, Mon, Tue, Wed, Thu, Fri, Sat])
    let dow_idx = ((days_since_epoch % 7 + 4) % 7 + 7) % 7;
    let days_of_week = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    let day_name = days_of_week[dow_idx as usize].to_string();

    let date_str = format!("{:04}-{:02}-{:02}", year, month, day);
    (date_str, day_name)
}

fn days_to_ymd(days: i64) -> (i32, u32, u32) {
    let z = days + 719468;
    let era = (if z >= 0 { z } else { z - 146096 }) / 146097;
    let doe = (z - era * 146097) as u32;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = (yoe as i64) + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y as i32, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_conversion() {
        // 2026-08-20 00:00:00 UTC = 1787184000
        let (date, day) = timestamp_to_date(1787184000);
        assert_eq!(date, "2026-08-20");
        assert_eq!(day, "Thu");

        // 1970-01-01
        let (date_epoch, day_epoch) = timestamp_to_date(0);
        assert_eq!(date_epoch, "1970-01-01");
        assert_eq!(day_epoch, "Thu");
    }

    #[test]
    fn test_compute_stats_and_streaks() {
        let now_secs = 1787184000; // 2026-08-20 (Thursday)
        let records = vec![
            BreakRecord {
                timestamp: now_secs - 86400 * 2, // Tuesday
                date: "2026-08-18".to_string(),
                action: "completed".to_string(),
                duration_seconds: 20,
            },
            BreakRecord {
                timestamp: now_secs - 86400, // Wednesday
                date: "2026-08-19".to_string(),
                action: "completed".to_string(),
                duration_seconds: 20,
            },
            BreakRecord {
                timestamp: now_secs - 3600, // Thursday (Today) #1
                date: "2026-08-20".to_string(),
                action: "completed".to_string(),
                duration_seconds: 20,
            },
            BreakRecord {
                timestamp: now_secs - 1800, // Thursday (Today) #2
                date: "2026-08-20".to_string(),
                action: "dismissed".to_string(),
                duration_seconds: 20,
            },
        ];

        let stats = HistoryManager::compute_stats(&records, now_secs);
        assert_eq!(stats.breaks_today, 2);
        assert_eq!(stats.breaks_this_week, 4);
        assert_eq!(stats.current_streak, 4);
        assert_eq!(stats.best_streak, 4);
        assert_eq!(stats.last_7_days.len(), 7);
        assert_eq!(stats.last_7_days.last().unwrap().count, 2); // Today's count
    }

    #[test]
    fn test_90_day_retention_pruning() {
        let temp_dir = std::env::temp_dir().join(format!("blink_test_hist_{}", rand::random::<u32>()));
        let hist_path = temp_dir.join("history.json");
        let mgr = HistoryManager::new_with_path(hist_path.clone());

        // Add an old record (100 days old)
        let now_secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let old_ts = now_secs - (100 * 86400);

        if let Ok(mut recs) = mgr.records.write() {
            recs.push(BreakRecord {
                timestamp: old_ts,
                date: "2020-01-01".to_string(),
                action: "completed".to_string(),
                duration_seconds: 20,
            });
        }

        // Record a new break -> triggers retention filter
        mgr.record_break("completed", 20).unwrap();

        let recs = mgr.records.read().unwrap();
        assert_eq!(recs.len(), 1);
        assert_ne!(recs[0].timestamp, old_ts);

        let _ = fs::remove_dir_all(temp_dir);
    }
}
