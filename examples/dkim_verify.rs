/*
 * SPDX-FileCopyrightText: 2020 Stalwart Labs LLC <hello@stalw.art>
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

use mail_auth::{AuthenticatedMessage, DkimResult, MessageAuthenticator};

const TEST_MESSAGE: &str = r#"DKIM-Signature: v=1; a=ed25519-sha256; c=relaxed/relaxed;
d=football.example.com; i=@football.example.com;
q=dns/txt; s=brisbane; t=1528637909; h=from : to :
subject : date : message-id : from : subject : date;
bh=2jUSOH9NhtVGCQWNr9BrIAPreKQjO6Sn7XIkfJVOzv8=;
b=/gCrinpcQOoIfuHNQIbq4pgh9kyIK3AQUdt9OdqQehSwhEIug4D11Bus
Fa3bT3FY5OsU7ZbnKELq+eXdp1Q1Dw==
DKIM-Signature: v=1; a=rsa-sha256; c=relaxed/relaxed;
d=football.example.com; i=@football.example.com;
q=dns/txt; s=test; t=1528637909; h=from : to : subject :
date : message-id : from : subject : date;
bh=2jUSOH9NhtVGCQWNr9BrIAPreKQjO6Sn7XIkfJVOzv8=;
b=F45dVWDfMbQDGHJFlXUNB2HKfbCeLRyhDXgFpEL8GwpsRe0IeIixNTe3
DhCVlUrSjV4BwcVcOF6+FF3Zo9Rpo1tFOeS9mPYQTnGdaSGsgeefOsk2Jz
dA+L10TeYt9BgDfQNZtKdN1WO//KgIqXP7OdEFE4LjFYNcUxZQ4FADY+8=
From: Joe SixPack <joe@football.example.com>
To: Suzie Q <suzie@shopping.example.net>
Subject: Is dinner ready?
Date: Fri, 11 Jul 2003 21:00:37 -0700 (PDT)
Message-ID: <20030712040037.46341.5F8J@football.example.com>

Hi.

We lost the game.  Are you hungry yet?

Joe."#;

#[tokio::main]
async fn main() {
    // Create an authenticator using Cloudflare DNS
    let authenticator = MessageAuthenticator::new_cloudflare_tls().unwrap();

    // Parse message
    let authenticated_message = AuthenticatedMessage::parse(TEST_MESSAGE.as_bytes()).unwrap();

    // Validate signature
    let result = authenticator.verify_dkim(&authenticated_message).await;

    // Make sure all signatures passed verification
    assert!(result.iter().all(|s| s.result() == &DkimResult::Pass));
}
