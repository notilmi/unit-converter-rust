mod domain;

use domain::{Area, Conversion, Unit};

fn main() {
    println!("============================================================");
    println!("        📐 Unit Converter - Area Domain Preview            ");
    println!("============================================================\n");

    let units = vec![
        Unit::new("sqkm"), // Square Kilometer
        Unit::new("ha"),   // Hectare
        Unit::new("sqm"),  // Square Meter (base metric)
        Unit::new("sqcm"), // Square Centimeter
        Unit::new("acre"), // Acre
        Unit::new("sqft"), // Square Foot (base imperial)
        Unit::new("sqin"), // Square Inch
    ];

    println!("[1] Registering Units:");
    for u in &units {
        println!("  • {}", u.name);
    }
    println!();

    let mut area = Area::initialize(units);

    // Note: add_bidirectional_edge(A, B, factor) means: B = A * factor
    println!("[2] Establishing Conversion Relationships (Edges):");

    // Metric conversions
    area.add_bidirectional_edge("sqkm", "sqm", 1_000_000.0);
    println!("  ✓ 1 sqkm = 1,000,000 sqm");

    area.add_bidirectional_edge("ha", "sqm", 10_000.0);
    println!("  ✓ 1 ha   = 10,000 sqm");

    area.add_bidirectional_edge("sqm", "sqcm", 10_000.0);
    println!("  ✓ 1 sqm  = 10,000 sqcm");

    // Metric <-> Imperial bridge (also demonstrates custom Conversion struct)
    let sqm_to_sqft = Conversion::new(10.7639104167);
    area.add_edge("sqm", "sqft", sqm_to_sqft);
    area.add_edge("sqft", "sqm", sqm_to_sqft.inverse());
    println!("  ✓ 1 sqm  ≈ 10.7639 sqft");

    // Imperial conversions
    area.add_bidirectional_edge("acre", "sqft", 43_560.0);
    println!("  ✓ 1 acre = 43,560 sqft");

    area.add_bidirectional_edge("sqft", "sqin", 144.0);
    println!("  ✓ 1 sqft = 144 sqin");
    println!();

    println!("[3] Topology & Neighbors Check:");
    let sample_unit = "sqm";
    let neighbors: Vec<String> = area
        .get_neighbor(sample_unit)
        .iter()
        .map(|u| u.name.clone())
        .collect();
    println!("  • Direct (1-hop) neighbors of '{}': {:?}", sample_unit, neighbors);

    let reachable: Vec<String> = area
        .get_reachable(sample_unit)
        .iter()
        .map(|u| u.name.clone())
        .collect();
    println!("  • All reachable units from '{}': {:?}", sample_unit, reachable);
    println!("  • Is 'sqft' a direct neighbor of 'sqm'? {}", area.is_neighbor("sqm", "sqft"));
    println!("  • Is 'acre' a direct neighbor of 'sqm'? {} (it's 2 hops away)", area.is_neighbor("sqm", "acre"));
    println!("  • Is 'acre' reachable from 'sqm'? {}", area.is_reachable("sqm", "acre"));
    println!();

    println!("[4] Conversion Demonstrations:");
    let test_cases = vec![
        ("sqkm", "sqm", 2.5, "Direct metric scale-down"),
        ("sqm", "sqkm", 500_000.0, "Reverse metric scale-up"),
        ("sqm", "sqft", 100.0, "Direct bridge (Metric -> Imperial)"),
        ("acre", "sqm", 1.0, "Multi-hop (acre -> sqft -> sqm)"),
        ("ha", "acre", 2.47105, "Multi-hop (ha -> sqm -> sqft -> acre)"),
        ("sqin", "sqcm", 100.0, "Multi-hop (sqin -> sqft -> sqm -> sqcm)"),
        ("sqm", "sqm", 42.0, "Identity conversion (same unit)"),
    ];

    for (from, to, val, desc) in test_cases {
        print!("  • {:<35} : {:>10.4} {:<5} -> ", desc, val, from);
        match area.convert(from, to, val) {
            Some(result) => println!("{:>14.4} {}", result, to),
            None => println!("❌ No conversion path found"),
        }
    }
    println!();

    println!("[5] Error / Edge-Case Handling:");
    let invalid_tests = vec![
        ("sqm", "unknown_unit", 100.0),
        ("non_existent", "sqkm", 50.0),
    ];
    for (from, to, val) in invalid_tests {
        print!("  • Attempting {:<12} -> {:<12} : ", from, to);
        match area.convert(from, to, val) {
            Some(res) => println!("{:.4} {}", res, to),
            None => println!("None (Safely rejected invalid unit)"),
        }
    }

    println!("\n============================================================");
    println!("                    Preview Complete!                       ");
    println!("============================================================");
}
