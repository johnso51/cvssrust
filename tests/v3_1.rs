use cvssrust::{CVSSScore, MinorVersion, Severity, CVSS};

#[test]
fn test_parse_vectors_v3_1() {
    let vulns_v3_1 = [
        // https://nvd.nist.gov/vuln/detail/CVE-2020-0601
        (
            "CVSS:3.1/AV:N/AC:L/PR:N/UI:R/S:U/C:H/I:H/A:N",
            8.1,
            Severity::High,
            0.0,
            0.0,
        ),
        // https://nvd.nist.gov/vuln/detail/CVE-2014-0011
        (
            "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H",
            9.8,
            Severity::Critical,
            0.0,
            0.0,
        ),
        // https://www.first.org/cvss/user-guide#2-6-Formula-Changes
        // The Temporal Score for all vulnerabilities which have a Base Score of 2.5, 5.0 or 10.0,
        //     Exploit Code Maturity (E) of High (H), Remediation Level (RL) of Unavailable (U)
        //     and Report Confidence (RC) of Unknown (U) is 0.1 lower in CVSS v3.1 than for 3.0.
        // For example, the following metric combination has a
        //     Temporal Score of 4.7 in CVSS v3.0, but 4.6 in v3.1:
        (
            "CVSS:3.1/AV:P/AC:H/PR:L/UI:R/S:U/C:L/I:L/A:H/E:H/RL:U/RC:U",
            5.0,
            Severity::Medium,
            0.0,
            0.0,
        ),
    ];

    for (vector, expected_base_score, expected_severity, expected_temp_score, expected_env_score) in
        vulns_v3_1.iter()
    {
        if let Ok(CVSS::V3(cvss)) = CVSS::parse(vector) {
            assert_eq!(cvss.to_string(), String::from(*vector));
            assert_eq!(cvss.minor_version, MinorVersion::V1);
            assert_eq!(cvss.base_score().value(), *expected_base_score);
            assert_eq!(cvss.base_score().severity(), *expected_severity);
            assert_eq!(cvss.temporal_score().value(), *expected_temp_score);
            assert_eq!(cvss.environmental_score().value(), *expected_env_score);
        } else {
            panic!("Parsing error")
        }
    }
}