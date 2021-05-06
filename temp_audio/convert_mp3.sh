!#bin/sh
for file in *.wav; do lame -V2 "$file" "${file%.wav}".mp3; done
