docker buildx create --use
docker buildx build \
    -t $USER/tauri-cross:armv7-unknown-linux-gnueabihf \
    --output type=registry \
    --progress=plain \
    --platform linux/arm/v7 \
    .

