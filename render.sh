cargo run --release -- --print && \
yes | ffmpeg -framerate 50 -pattern_type glob -i 'out/*.png' -i kintaro.wav -c:a copy -shortest -c:v libx264 -r 50 -pix_fmt yuv420p out.mov \
&& open out.mov
