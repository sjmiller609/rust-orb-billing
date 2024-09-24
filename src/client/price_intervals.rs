// Copyright Materialize, Inc. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License in the LICENSE file at the
// root of this repository, or online at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::client::subscriptions::Subscription;
use crate::client::Client;
use crate::error::Error;
use reqwest::Method;
use serde::Serialize;
use time::OffsetDateTime;

/// The Price Interval resource represents a period of time for which a price will bill on a subscription. A subscription’s price intervals define its billing behavior.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct AddEditPriceIntervalParams<'a> {
    /// A list of adjustments to add to the subscription.
    #[serde(default = "Vec::new")]
    pub add_adjustments: Vec<AddAdjustmentIntervalParams<'a>>,
    /// Not implemented
    #[serde(default = "Vec::new")]
    pub add: Vec<()>,
    /// Not implemented
    #[serde(default = "Vec::new")]
    pub edit: Vec<()>,
    /// Not implemented
    #[serde(default = "Vec::new")]
    pub edit_adjustments: Vec<()>,
}

/// Parameters for adding a new adjustment interval to a subscription.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AddAdjustmentIntervalParams<'a> {
    /// The definition of a new adjustment to create and add to the subscription.
    pub adjustment: NewAdjustment<'a>,
    /// The start date of the adjustment interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "time::serde::rfc3339::option")]
    pub start_date: Option<OffsetDateTime>,
    /// The end date of the adjustment interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "time::serde::rfc3339::option")]
    pub end_date: Option<OffsetDateTime>,
}

/// Represents a new adjustment to be applied to a subscription.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "adjustment_type")]
pub enum NewAdjustment<'a> {
    /// A percentage discount adjustment.
    #[serde(rename = "percentage_discount")]
    PercentageDiscount {
        /// The IDs of the prices to which this discount applies.
        applies_to_price_ids: Vec<&'a str>,
        /// The percentage discount to apply, represented as a decimal (e.g., 0.1 for 10%).
        percentage_discount: f64,
    },
    /// An amount discount adjustment.
    #[serde(rename = "amount_discount")]
    AmountDiscount {
        /// The IDs of the prices to which this discount applies.
        applies_to_price_ids: Vec<&'a str>,
        /// The fixed amount to discount, represented as a string (e.g., "10.00").
        amount_discount: &'a str,
    },
}

impl Client {
    /// Add or edit price intervals for a subscription.
    ///
    /// This endpoint is used to add and edit subscription price intervals. By making modifications
    /// to a subscription's price intervals, you can flexibly and atomically control the billing
    /// behavior of a subscription.
    pub async fn add_edit_price_intervals(
        &self,
        subscription_id: &str,
        params: &AddEditPriceIntervalParams<'_>,
    ) -> Result<Subscription, Error> {
        let req = self.build_request(
            Method::POST,
            ["subscriptions", subscription_id, "price_intervals"],
        );
        let req = req.json(params);
        let res = self.send_request(req).await?;
        Ok(res)
    }
}
