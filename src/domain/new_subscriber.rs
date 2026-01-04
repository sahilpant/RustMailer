use serde::Deserialize;

use crate::domain::subscriber_name::SubscriberName;

#[derive(Deserialize)]
pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName
}