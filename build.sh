xcrun -sdk macosx metal -c src/metal/find_seed.metal -o src/metal/find_seed.air
xcrun -sdk macosx metallib src/metal/find_seed.air -o src/metal/find_seed.metallib