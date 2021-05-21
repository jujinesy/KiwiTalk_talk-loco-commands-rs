/*
 * Created on Tue Dec 01 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::BsonData;

/// Request checkin server information
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct GetConf {

    /// Network MCCMNC
    #[serde(rename = "MCCMNC")]
    pub mccmnc: String,

    /// Current OS (win32, android, mac, etc.)
    pub os: String,

    /// Device model (mobile only) leave it empty if it's not mobile device.
    pub model: String

}