cargo run --release -- --print
# yes | ffmpeg -framerate 40 -pattern_type glob -i 'out/*.png' -i kintaro.wav -c:a copy -shortest -c:v libx264 -r 40 -pix_fmt yuv420p out.mov 
