# echo-jni

If you haven't installed rust android targets first add those to your environment using [rustup](https://www.rust-lang.org/learn/get-started)

   ```
   rustup target add x86_64-apple-darwin x86_64-unknown-linux-gnu x86_64-linux-android aarch64-linux-android armv7-linux-androideabi i686-linux-android
   ```

Then make sure that you have an Android NDK installed (preferably the latest one), and that you have an `NDK_HOME` env variable set before you start building the library. Usually, if installed through the `sdkmanager`,
your `NDK_HOME` will look more or less like this: `/home/user/Android/Sdk/ndk/<version>/`.

Build android library in `.aar` format with:
```
./gradlew build
```
Gradle will build automatically the native library with rust for all 4 platforms using NDK. You can choose to build only for a specific platform by setting the env variable `BUILD_TARGETS` to a comma-separated list
containing one or more of the following items:

* `aarch64`
* `armv7`
* `x86_64`
* `i686`

The output aar library is available at `./library/build/outputs/aar`.

You can run the tests with:
```
./gradlew connectedDebugAndroidTest
```