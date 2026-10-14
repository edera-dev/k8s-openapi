// Generated from Go type k8s.io/api/certificates/v1.KeyUsage

crate::string_enum! {
    /// KeyUsage specifies valid usage contexts for keys.
    /// See:
    ///
    ///   https://tools.ietf.org/html/rfc5280#section-4.2.1.3
    ///     https://tools.ietf.org/html/rfc5280#section-4.2.1.12
    ///
    /// Used by:
    ///
    /// - [`CertificateSigningRequestSpec::usages`](crate::api::certificates::v1::CertificateSigningRequestSpec::usages)
    pub enum KeyUsage {
        Signing = "signing",

        DigitalSignature = "digital signature",

        ContentCommitment = "content commitment",

        KeyEncipherment = "key encipherment",

        KeyAgreement = "key agreement",

        DataEncipherment = "data encipherment",

        CertSign = "cert sign",

        CRLSign = "crl sign",

        EncipherOnly = "encipher only",

        DecipherOnly = "decipher only",

        Any = "any",

        ServerAuth = "server auth",

        ClientAuth = "client auth",

        CodeSigning = "code signing",

        EmailProtection = "email protection",

        SMIME = "s/mime",

        IPsecEndSystem = "ipsec end system",

        IPsecTunnel = "ipsec tunnel",

        IPsecUser = "ipsec user",

        Timestamping = "timestamping",

        OCSPSigning = "ocsp signing",

        MicrosoftSGC = "microsoft sgc",

        NetscapeSGC = "netscape sgc",
    }
}
