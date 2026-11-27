//! Generated gateway contract types, from the OpenRPC document.

mod generated;

pub use generated::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventories_round_trip() {
        assert_eq!(Method::ALL.len(), METHOD_COUNT);
        assert_eq!(Notification::ALL.len(), NOTIFICATION_COUNT);
        assert_eq!(ServerRequest::ALL.len(), SERVER_REQUEST_COUNT);
        for method in Method::ALL {
            assert_eq!(Method::parse(method.as_str()), Some(*method));
        }
        for notification in Notification::ALL {
            assert_eq!(
                Notification::parse(notification.as_str()),
                Some(*notification)
            );
        }
        for request in ServerRequest::ALL {
            assert_eq!(ServerRequest::parse(request.as_str()), Some(*request));
        }
        assert_eq!(Method::parse("no.such.method"), None);
    }
}
