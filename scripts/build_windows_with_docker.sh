cd ..
docker build -f ./scripts/windows-build.Dockerfile . -t rust_cross_compile_for_windows
docker run --rm -v ${PWD}/:/app rust_cross_compile_for_windows