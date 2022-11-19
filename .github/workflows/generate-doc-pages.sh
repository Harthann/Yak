cargo doc --no-deps
rm -rf ./docs
echo "<meta http-equiv=\"refresh\" content=\"0; url=build_wheel\">" > target/i386/index.html
cp -r target/i386/ docs
