#!/bin/sh
#for f in *.txt.json; do jq "[.scenes | .[] | .texts | .[] | {speaker: .[0], line: .[1] | .[] | .[1], voice: .[2] | .[0] | .voice}]| map(select((.voice != null)))" $f; done > all.json
#OR better because merges arrays
jq "[.scenes | .[] | .texts | .[] | {speaker: .[0], line: .[1] | .[] | .[1], voice: .[2] | .[0] | .voice}]| map(select((.voice != null)))" *.txt.json | jq -s > all.json
