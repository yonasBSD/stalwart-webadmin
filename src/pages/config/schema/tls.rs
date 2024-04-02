use crate::core::schema::*;

impl Builder<Schemas, ()> {
    pub fn build_tls(self) -> Self {
        self.new_schema("acme")
            .names("ACME provider", "ACME providers")
            .prefix("acme")
            .suffix("directory")
            // Id
            .new_id_field()
            .label("Directory Id")
            .help("Unique identifier for the ACME provider")
            .build()
            // Directory
            .new_field("directory")
            .label("Directory URL")
            .help("The URL of the ACME directory endpoint")
            .typ(Type::Input)
            .input_check([Transformer::Trim], [Validator::Required, Validator::IsUrl])
            .default("https://acme-v02.api.letsencrypt.org/directory")
            .build()
            // Domains
            .new_field("domains")
            .typ(Type::Array)
            .input_check([Transformer::Trim], [Validator::Required])
            .label("Domains")
            .help("Domains covered by this ACME manager")
            .build()
            // Default provider
            .new_field("default")
            .typ(Type::Boolean)
            .label("Default provider")
            .help(concat!(
                "Whether the certificates generated by this provider ",
                "should be the default when no SNI is provided"
            ))
            .build()
            // Contact
            .new_field("contact")
            .label("Contact Email")
            .help(concat!(
                "the contact email address, which is used for important ",
                "communications regarding your ACME account and certificates"
            ))
            .typ(Type::Array)
            .input_check(
                [Transformer::Trim],
                [Validator::Required, Validator::IsEmail],
            )
            .build()
            // Renew before
            .new_field("renew-before")
            .typ(Type::Duration)
            .label("Renew before")
            .help("Determines how early before expiration the certificate should be renewed.")
            .input_check([Transformer::Trim], [Validator::Required])
            .default("30d")
            .build()
            // Account key
            .new_field("account-key")
            .label("Account key")
            .help(concat!(
                "The account key used to authenticate with the ACME ",
                "provider (auto-generated)"
            ))
            .typ(Type::Secret)
            .build()
            // Account key
            .new_field("cert")
            .label("TLS Certificate")
            .help(concat!(
                "The TLS certificate generated by the ACME provider ",
                "(auto-generated, do not modify)"
            ))
            .typ(Type::Secret)
            .build()
            // Lists
            .list_title("ACME providers")
            .list_subtitle("Manage ACME TLS certificate providers")
            .list_fields(["_id", "contact", "renew-before", "default"])
            // Form
            .new_form_section()
            .title("ACME provider")
            .fields([
                "_id",
                "directory",
                "contact",
                "domains",
                "renew-before",
                "default",
            ])
            .build()
            .new_form_section()
            .title("Certificate")
            .fields(["account-key", "cert"])
            .build()
            .build()
            // ---- TLS certificates ----
            .new_schema("certificate")
            .reload_prefix("certificate")
            .names("certificate", "certificates")
            .prefix("certificate")
            .suffix("cert")
            // Id
            .new_id_field()
            .label("Certificate Id")
            .help("Unique identifier for the TLS certificate")
            .build()
            // Default provider
            .new_field("default")
            .typ(Type::Boolean)
            .label("Default certificate")
            .help(concat!(
                "Whether this certificate ",
                "should be the default when no SNI is provided"
            ))
            .build()
            // Cert
            .new_field("cert")
            .label("Certificate")
            .typ(Type::Text)
            .help("TLS certificate in PEM format")
            .input_check([Transformer::Trim], [Validator::Required])
            .build()
            // PK
            .new_field("private-key")
            .label("Private Key")
            .typ(Type::Text)
            .help("Private key in PEM format")
            .input_check([Transformer::Trim], [Validator::Required])
            .build()
            .new_field("subjects")
            .typ(Type::Array)
            .input_check([Transformer::Trim], [Validator::IsDomain])
            .label("Subject Alternative Names")
            .help("Subject Alternative Names (SAN) for the certificate")
            .build()
            .list_title("TLS certificates")
            .list_subtitle("Manage TLS certificates")
            .list_fields(["_id", "subjects", "default"])
            .new_form_section()
            .title("TLS certificate")
            .fields(["_id", "cert", "private-key", "subjects", "default"])
            .build()
            .build()
            // ---- TLS settings ----
            .new_schema("tls")
            // TLS fields
            .add_tls_fields(false)
            // Forms
            .new_form_section()
            .title("Default TLS options")
            .fields([
                "server.tls.disable-protocols",
                "server.tls.disable-ciphers",
                "server.tls.timeout",
                "server.tls.ignore-client-order",
            ])
            .build()
            .build()
    }
}

impl Builder<Schemas, Schema> {
    pub fn add_tls_fields(self, is_listener: bool) -> Self {
        let do_override: &'static [&'static str] =
            if is_listener { &["true"][..] } else { &[][..] };

        // Ignore client order
        self.new_field(if is_listener {
            "tls.ignore-client-order"
        } else {
            "server.tls.ignore-client-order"
        })
        .label("Ignore client order")
        .help("Whether to ignore the client's cipher order")
        .typ(Type::Boolean)
        .default("true")
        .display_if_eq("tls.override", do_override.iter().copied())
        .build()
        // Timeout
        .new_field(if is_listener {
            "tls.timeout"
        } else {
            "server.tls.timeout"
        })
        .label("Handshake Timeout")
        .help("TLS handshake timeout")
        .typ(Type::Duration)
        .default("1m")
        .display_if_eq("tls.override", do_override.iter().copied())
        .build()
        // Protocols
        .new_field(if is_listener {
            "tls.disable-protocols"
        } else {
            "server.tls.disable-protocols"
        })
        .label("Disabled Protocols")
        .help("Which TLS protocols to disable")
        .typ(Type::Select {
            multi: true,
            source: Source::Static(TLS_PROTOCOLS),
        })
        .display_if_eq("tls.override", do_override.iter().copied())
        .build()
        // Ciphersuites
        .new_field(if is_listener {
            "tls.disable-ciphers"
        } else {
            "server.tls.disable-ciphers"
        })
        .label("Disabled Ciphersuites")
        .help("Which ciphersuites to disable")
        .typ(Type::Select {
            multi: true,
            source: Source::Static(TLS_CIPHERSUITES),
        })
        .display_if_eq("tls.override", do_override.iter().copied())
        .build()
    }
}

pub static TLS_PROTOCOLS: &[(&str, &str)] = &[
    ("TLSv1.2", "TLS version 1.2"),
    ("TLSv1.3", "TLS version 1.3"),
];

pub static TLS_CIPHERSUITES: &[(&str, &str)] = &[
    ("TLS13_AES_256_GCM_SHA384", "TLS1.3 AES256 GCM SHA384"),
    ("TLS13_AES_128_GCM_SHA256", "TLS1.3 AES128 GCM SHA256"),
    (
        "TLS13_CHACHA20_POLY1305_SHA256",
        "TLS1.3 CHACHA20 POLY1305 SHA256",
    ),
    (
        "TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384",
        "ECDHE ECDSA AES256 GCM SHA384",
    ),
    (
        "TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256",
        "ECDHE ECDSA AES128 GCM SHA256",
    ),
    (
        "TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256",
        "ECDHE ECDSA CHACHA20 POLY1305 SHA256",
    ),
    (
        "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384",
        "ECDHE RSA AES256 GCM SHA384",
    ),
    (
        "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256",
        "ECDHE RSA AES128 GCM SHA256",
    ),
    (
        "TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256",
        "ECDHE RSA CHACHA20 POLY1305 SHA256",
    ),
];