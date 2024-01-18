use serde::{Deserialize, Serialize};

use crate::types::Integer;

/// Contains information about Telegram Passport data shared with the bot by the user.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PassportData {
    pub data: Vec<EncryptedPassportElement>,
    pub credentials: EncryptedCredentials,
}

/// This object represents a file uploaded to Telegram Passport. Currently all Telegram Passport files are in JPEG format when decrypted and don't exceed 10MB.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PassportFile {
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: Integer,
    pub file_date: Integer,
}

/// Contains information about documents or other Telegram Passport elements shared with the bot by the user.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EncryptedPassportElement {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<PassportFile>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front_side: Option<PassportFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_side: Option<PassportFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selfie: Option<PassportFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<Vec<PassportFile>>,
    pub hash: String,
}

/// Contains data required for decrypting and authenticating EncryptedPassportElement. See the Telegram Passport Documentation for a complete description of the data decryption and authentication processes.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EncryptedCredentials {
    pub data: String,
    pub hash: String,
    pub secret: String,
}

/// This object represents an error in the Telegram Passport element which was submitted that should be resolved by the user. It should be one of:
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum PassportElementError {
    PassportElementErrorDataField,
    PassportElementErrorFrontSide,
    PassportElementErrorReverseSide,
    PassportElementErrorSelfie,
    PassportElementErrorFile,
    PassportElementErrorFiles,
    PassportElementErrorTranslationFile,
    PassportElementErrorTranslationFiles,
    PassportElementErrorUnspecified,
}

/// Represents an issue in one of the data fields that was provided by the user. The error is considered resolved when the field's value changes.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PassportElementErrorDataField {
    pub source: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub field_name: String,
    pub data_hash: String,
    pub message: String,
}

/// Represents an issue with the front side of a document. The error is considered resolved when the file with the front side of the document changes.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PassportElementErrorFrontSide {
    pub source: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub file_hash: String,
    pub message: String,
}

/// Represents an issue with the reverse side of a document. The error is considered resolved when the file with reverse side of the document changes.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PassportElementErrorReverseSide {
    pub source: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub file_hash: String,
    pub message: String,
}

/// Represents an issue with the selfie with a document. The error is considered resolved when the file with the selfie changes.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PassportElementErrorSelfie {
    pub source: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub file_hash: String,
    pub message: String,
}

/// Represents an issue with a document scan. The error is considered resolved when the file with the document scan changes.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PassportElementErrorFile {
    pub source: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub file_hash: String,
    pub message: String,
}

/// Represents an issue with a list of scans. The error is considered resolved when the list of files containing the scans changes.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PassportElementErrorFiles {
    pub source: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub file_hash: String,
    pub message: String,
}

/// Represents an issue with one of the files that constitute the translation of a document. The error is considered resolved when the file changes.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PassportElementErrorTranslationFile {
    pub source: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub file_hash: String,
    pub message: String,
}

/// Represents an issue with the translated version of a document. The error is considered resolved when a file with the document translation change.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PassportElementErrorTranslationFiles {
    pub source: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub file_hash: String,
    pub message: String,
}

/// Represents an issue in an unspecified place. The error is considered resolved when new data is added.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PassportElementErrorUnspecified {
    pub source: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub file_hash: String,
    pub message: String,
}
