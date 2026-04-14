use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Use any `.app` path here. If the app is translocated, this returns
    // the original bundle path.
    let app_bundle_path = std::env::current_exe()?;
    let resolved = untranslocator::resolve_translocated_path(&app_bundle_path)?;

    println!("Input path:   {}", app_bundle_path.as_path().display());
    println!("Resolved path: {}", resolved.display());
    Ok(())
}
