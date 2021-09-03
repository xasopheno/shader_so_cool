cargo run --release && \
yes | ffmpeg -framerate 10 -pattern_type glob -i 'out/*.png' -i kintaro.wav -c:a copy -shortest -c:v libx264 -r 30 -pix_fmt yuv420p out.mov \
&& open out.mov
