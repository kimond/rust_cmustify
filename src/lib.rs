extern crate notify_rust;

use std::collections::HashMap;
use notify_rust::Notification;

const BREAK_TAG: &'static str = "!break!";

const VALID_TAGS: [&'static str; 11] = ["status", "url", "file", "artist", "album", "discnumber", "tracknumber", "title", "date", "duration", BREAK_TAG];

pub type Metadata = HashMap<String, String>;

pub fn print_usage() {
    println!("You must set cmus to call this script as notifier");
}

pub fn parse(cmus_data: String) -> Metadata {
    let mut result = Metadata::new();
    let mut split_data: Vec<&str> = cmus_data.split(" ").collect();
    split_data.push(BREAK_TAG);
    let mut last_tag_found: Option<&str> = None;
    let mut value_collector: Vec<&str> = vec![];
    for part in split_data.iter() {
        if VALID_TAGS.contains(part) {
            if value_collector.len() > 0 {
                if let Some(tag) = last_tag_found {
                    result.insert(tag.to_string(), value_collector.join(" "));
                    value_collector.clear();
                }
            }
            last_tag_found = Some(part);
            continue;
        }
        value_collector.push(part);
    }
    result
}

pub fn format_notification_body(m: &Metadata) -> String {
    let mut notification_body = match m.get("title") {
        Some(t) => t.to_string(),
        None => "Unknown".to_string()
    };

    if let Some(artist) = m.get("artist") {
        notification_body = format!("{} by {}", notification_body, artist);
        if let Some(album) = m.get("album") {
            notification_body = format!("{}, {}", notification_body, album);
        }
    }

    notification_body
}

pub trait Notifier {
    fn send(&self, summary: String, content: String);
}

pub struct DbusNotifier { }

impl Notifier for DbusNotifier {
    fn send(&self, summary: String, content: String) {
        Notification::new()
            .summary(&summary)
            .body(&content)
            .show().unwrap();
    }
}

pub fn run<T>(n: &T, cmus_data: String)
    where T: Notifier {
    let metadata = parse(cmus_data);
    let notification_body = format_notification_body(&metadata);
    n.send("Cmustify - Current song".to_string(), notification_body);
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[test]
    fn parse_cmus_data_correctly() {
        let cmus_data = "artist Todd album Reno title super song".to_string();
        let result = parse(cmus_data);

        assert_eq!(result.get("artist").unwrap(), "Todd");
        assert_eq!(result.get("album").unwrap(), "Reno");
        assert_eq!(result.get("title").unwrap(), "super song");
    }

    #[test]
    fn format_notification_body_correctly() {
        let mut metadata = Metadata::new();
        metadata.insert("title".to_string(), "Super title".to_string());
        metadata.insert("artist".to_string(), "Todd".to_string());

        let notification_body = format_notification_body(&metadata);

        assert!(notification_body.contains("Super title"));
        assert!(notification_body.contains("Todd"));
    }

    #[test]
    fn run_send_notification() {
        struct NotifyMock {
            called: RefCell<bool>
        }

        impl Notifier for NotifyMock {
            fn send(&self, _summary: String, _content: String) {
                *self.called.borrow_mut() = true;
            }
        }

        let notifier_mock = NotifyMock { called: RefCell::new(false) };
        let notification_body = "Hola".to_string();
        run(&notifier_mock, notification_body);

        assert!(*notifier_mock.called.borrow())
    }
}