/*
 * Software Name : libits-client
 * SPDX-FileCopyrightText: Copyright (c) Orange SA
 * SPDX-License-Identifier: MIT
 *
 * This software is distributed under the MIT license,
 * see the "LICENSE.txt" file for more details or https://opensource.org/license/MIT/
 *
 * Authors: see CONTRIBUTORS.md
 */

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigurationError {
    #[error("{0}")]
    BootstrapFailure(String),
    #[error("Could not found field '{0}'")]
    FieldNotFound(&'static str),
    #[error("Cannot parse '{0}' due to invalid file type")]
    InvalidFileType(String),
    #[error("Configuration missing mandatory field {0} in section {1}")]
    MissingMandatoryField(&'static str, &'static str),
    #[error("Configuration missing mandatory section: {0}")]
    MissingMandatorySection(&'static str),
    #[error("No custom settings found in configuration")]
    NoCustomSettings,
    #[error("Could not found section '{0}'")]
    SectionNotFound(&'static str),
    #[error("Could not parse value of field '{0}' as a '{1}'")]
    TypeError(&'static str, &'static str),
    #[error("Username provided with no password")]
    NoPassword,
}
