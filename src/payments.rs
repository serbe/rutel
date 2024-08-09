use serde::{Deserialize, Serialize};

use crate::{
    files::{PhotoSize, Video},
    types::{Integer, User},
};

/// This object represents a portion of the price for goods or services.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LabeledPrice {
    pub label: String,
    pub amount: Integer,
}

/// This object contains basic information about an invoice.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Invoice {
    pub title: String,
    pub description: String,
    pub start_parameter: String,
    pub currency: String,
    pub total_amount: Integer,
}

/// This object represents a shipping address.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ShippingAddress {
    pub country_code: String,
    pub state: String,
    pub city: String,
    pub street_line1: String,
    pub street_line2: String,
    pub post_code: String,
}

/// This object represents information about an order.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct OrderInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<ShippingAddress>,
}

/// This object represents one shipping option.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ShippingOption {
    pub id: String,
    pub title: String,
    pub prices: Vec<LabeledPrice>,
}

/// This object contains basic information about a successful payment.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SuccessfulPayment {
    pub currency: String,
    pub total_amount: Integer,
    pub invoice_payload: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_info: Option<OrderInfo>,
    pub telegram_payment_charge_id: String,
    pub provider_payment_charge_id: String,
}

/// This object contains basic information about a refunded payment.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RefundedPayment {
    /// Three-letter ISO 4217 currency code, or “XTR” for payments in Telegram Stars. Currently, always “XTR”
    pub currency: String,
    /// Total refunded price in the smallest units of the currency (integer, not float/double). For example, for a price of US$ 1.45, total_amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub total_amount: Integer,
    /// Bot-specified invoice payload
    pub invoice_payload: String,
    /// Telegram payment identifier
    pub telegram_payment_charge_id: String,
    /// Optional. Provider payment identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_payment_charge_id: Option<String>,
}

/// This object contains information about an incoming shipping query.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ShippingQuery {
    pub id: String,
    pub from: User,
    pub invoice_payload: String,
    pub shipping_address: ShippingAddress,
}

/// This object contains information about an incoming pre-checkout query.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PreCheckoutQuery {
    pub id: String,
    pub from: User,
    pub currency: String,
    pub total_amount: Integer,
    pub invoice_payload: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_info: Option<OrderInfo>,
}

/// This object describes the state of a revenue withdrawal operation. Currently, it can be one of
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum RevenueWithdrawalState {
    RevenueWithdrawalStatePending,
    RevenueWithdrawalStateSucceeded,
    RevenueWithdrawalStateFailed,
}

/// The withdrawal is in progress.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RevenueWithdrawalStatePending {
    /// Type of the state, always “pending”
    #[serde(rename = "type")]
    pub kind: String,
}

/// The withdrawal succeeded.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RevenueWithdrawalStateSucceeded {
    /// Type of the state, always “succeeded”
    #[serde(rename = "type")]
    pub kind: String,
    /// Date the withdrawal was completed in Unix time
    pub date: Integer,
    /// An HTTPS URL that can be used to see transaction details
    pub url: String,
}

/// The withdrawal failed and the transaction was refunded.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RevenueWithdrawalStateFailed {
    /// Type of the state, always “failed”
    #[serde(rename = "type")]
    pub kind: String,
}

/// This object describes the source of a transaction, or its recipient for outgoing transactions. Currently, it can be one of
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum TransactionPartner {
    TransactionPartnerUser,
    TransactionPartnerFragment,
    TransactionPartnerTelegramAds,
    TransactionPartnerOther,
}

/// Describes a transaction with a user.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TransactionPartnerUser {
    /// Type of the transaction partner, always “user”
    #[serde(rename = "type")]
    pub kind: String,
    /// Information about the user
    pub user: User,
    /// Optional. Bot-specified invoice payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_payload: Option<String>,
}

/// Describes a withdrawal transaction with Fragment.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TransactionPartnerFragment {
    /// Type of the transaction partner, always “fragment”
    #[serde(rename = "type")]
    pub kind: String,
    /// Optional. State of the transaction if the transaction is outgoing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withdrawal_state: Option<RevenueWithdrawalState>,
}

/// Describes a withdrawal transaction to the Telegram Ads platform.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TransactionPartnerTelegramAds {
    /// Type of the transaction partner, always “telegram_ads”
    #[serde(rename = "type")]
    pub kind: String,
}

/// Describes a transaction with an unknown source or recipient.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TransactionPartnerOther {
    /// Type of the transaction partner, always “other”
    #[serde(rename = "type")]
    pub kind: String,
}

/// Describes a Telegram Star transaction.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StarTransaction {
    /// Unique identifier of the transaction. Coincides with the identifer of the original transaction for refund transactions. Coincides with SuccessfulPayment.telegram_payment_charge_id for successful incoming payments from users.
    pub id: String,
    /// Number of Telegram Stars transferred by the transaction
    pub amount: Integer,
    /// Date the transaction was created in Unix time
    pub date: Integer,
    /// Optional. Source of an incoming transaction (e.g., a user purchasing goods or services, Fragment refunding a failed withdrawal). Only for incoming transactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<TransactionPartner>,
    /// Optional. Receiver of an outgoing transaction (e.g., a user for a purchase refund, Fragment for a withdrawal). Only for outgoing transactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver: Option<TransactionPartner>,
}

/// Contains a list of Telegram Star transactions.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StarTransactions {
    /// The list of transactions
    pub transactions: Vec<StarTransaction>,
}

/// Describes the paid media added to a message.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PaidMediaInfo {
    /// The number of Telegram Stars that must be paid to buy access to the media
    pub star_count: Integer,
    /// Information about the paid media
    pub paid_media: Vec<PaidMedia>,
}

/// This object describes paid media. Currently, it can be one of
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum PaidMedia {
    PaidMediaPreview,
    PaidMediaPhoto,
    PaidMediaVideo,
}

/// The paid media isn't available before the payment.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PaidMediaPreview {
    /// Type of the paid media, always “preview”
    #[serde(rename = "type")]
    pub kind: String,
    /// Optional. Media width as defined by the sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Integer>,
    /// Optional. Media height as defined by the sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Integer>,
    /// Optional. Duration of the media in seconds as defined by the sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Integer>,
}

/// The paid media is a photo.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PaidMediaPhoto {
    /// Type of the paid media, always “photo”
    #[serde(rename = "type")]
    pub kind: String,
    /// The photo
    pub photo: Vec<PhotoSize>,
}

/// The paid media is a video.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PaidMediaVideo {
    /// Type of the paid media, always “video”
    #[serde(rename = "type")]
    pub kind: String,
    /// The video
    pub video: Video,
}
