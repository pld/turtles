use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    // Only run the bundling logic when building for macOS in release mode
    if cfg!(target_os = "macos") && env::var("PROFILE").unwrap_or_default() == "release" {
        println!("cargo:rustc-env=TARGET_OS=macos");
        
        // Get the output directory
        let out_dir = env::var("OUT_DIR").unwrap();
        let target_dir = Path::new(&out_dir).ancestors().nth(3).unwrap();
        
        // Create the app bundle structure
        let bundle_dir = target_dir.join("ScreenSage.app");
        let contents_dir = bundle_dir.join("Contents");
        let macos_dir = contents_dir.join("MacOS");
        let resources_dir = contents_dir.join("Resources");
        
        // Create directories
        fs::create_dir_all(&macos_dir).unwrap();
        fs::create_dir_all(&resources_dir).unwrap();
        
        // Copy the executable if it exists
        // Note: During the build process, the executable might not exist yet
        let executable_path = target_dir.join("screensage");
        if executable_path.exists() {
            fs::copy(&executable_path, macos_dir.join("ScreenSage")).unwrap();
            
            // Set executable permissions
            if cfg!(unix) {
                Command::new("chmod")
                    .args(&["+x", macos_dir.join("ScreenSage").to_str().unwrap()])
                    .status()
                    .unwrap();
            }
        } else {
            println!("Executable not found at {:?} - it will be copied in a post-build step", executable_path);
        }
        
        // Create Info.plist
        let info_plist = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>CFBundleExecutable</key>
    <string>ScreenSage</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>CFBundleIdentifier</key>
    <string>com.example.screensage</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>ScreenSage</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.13</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSHumanReadableCopyright</key>
    <string>Copyright © 2023. All rights reserved.</string>
</dict>
</plist>"#;
        
        fs::write(contents_dir.join("Info.plist"), info_plist).unwrap();
        
        // Copy the icon if it exists
        let icon_path = Path::new("resources").join("AppIcon.icns");
        if icon_path.exists() {
            fs::copy(icon_path, resources_dir.join("AppIcon.icns")).unwrap();
        }
        
        
        println!("Created macOS application bundle at: {}", bundle_dir.display());
    }
}
