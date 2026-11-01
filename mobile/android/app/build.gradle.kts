// ElyMark's APK: gpui-mobile's activity around libely_mark.so.
import groovy.json.JsonSlurper

plugins {
    id("com.android.application")
}

val root = rootDir.resolve("../..")

// gpui-mobile's Java side, from the checkout Cargo uses.
fun gpuiMobileJava(): File {
    val out = providers.exec {
        commandLine("cargo", "metadata", "--format-version", "1", "--manifest-path", root.resolve("Cargo.toml").path)
    }.standardOutput.asText.get()
    @Suppress("UNCHECKED_CAST")
    val packages = (JsonSlurper().parseText(out) as Map<String, Any>)["packages"] as List<Map<String, Any>>
    val manifest = packages.first { it["name"] == "gpui-pre-mobile" }["manifest_path"] as String
    return File(manifest).resolveSibling("example/android/gradle/app/src/main/java")
}

// The launcher icon comes from the app's icon master.
val icon by tasks.registering(Copy::class) {
    from(root.resolve("assets/icons/ely-mark.png"))
    rename { "ic_launcher.png" }
    into(layout.buildDirectory.dir("generated/icon/mipmap"))
}

android {
    namespace = "com.elyoffice.mark"
    compileSdk = 34

    defaultConfig {
        applicationId = "com.elyoffice.mark"
        minSdk = 26
        targetSdk = 34
        versionCode = 1
        versionName = "0.1.13"
        ndk {
            abiFilters += listOf("arm64-v8a", "x86_64")
        }
        manifestPlaceholders["nativeLibraryName"] = "ely_mark"
    }

    sourceSets {
        getByName("main") {
            java.srcDir(gpuiMobileJava())
            res.srcDir(layout.buildDirectory.dir("generated/icon").get().asFile)
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }

    packaging {
        jniLibs {
            keepDebugSymbols += listOf("*/*/libely_mark.so")
        }
    }

    lint {
        abortOnError = false
        checkReleaseBuilds = false
    }
}

tasks.named("preBuild") { dependsOn(icon) }

// What gpui-mobile's Java classes import.
dependencies {
    implementation("androidx.core:core:1.12.0")
    implementation("androidx.core:core-splashscreen:1.0.1")
    implementation("androidx.biometric:biometric:1.1.0")
    implementation("androidx.media:media:1.7.1")
}
