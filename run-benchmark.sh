mkdir benchmarks-data 2> /dev/null

YEARS=(
    2025
    2024
    2023
    2022
    2021
    2020
    2019
    2018
    2017
    2016
    2015
)
site="<!DOCTYPE html><html><head><meta charset=\"UTF-8\" /><title>Advent of Code</title></head><body>"

emoji () {
    if [[ $1 == *"ns"* ]]; then
        echo "💅"
    elif [[ $1 == *"µs"* ]]; then
        echo "👑"
    elif [[ $1 == *"ms"* ]]; then
        echo "🥈"
    else 
        echo "🥉"
    fi
}

textSettings=""
for year in "${YEARS[@]}"; do
    height="830"
    cap="25"
    if [ "$year" -gt "2024" ]; then
        cap="12"
        height="440"
    fi

    output="<svg height=\"$height\" width=\"1300\" xmlns=\"http://www.w3.org/2000/svg\">"
    output="$output<style>"
    output="$output text { font-family: \"Arial\", monospace; }"
    output="$output rect { fill: transparent }"
    output="$output text { fill: black }"
    output="$output @media (prefers-color-scheme: dark) {"
    output="$output text { fill: white }"
    output="$output rect { fill: #0D1117 }"
    output="$output } </style>"
    output="$output<rect width=\"100%\" height=\"100%\" />"
    output="$output<text x=\"10\" y=\"30\" $textSettings>Year $year</text>"
    output="$output<text x=\"305\" y=\"30\" $textSettings>Part 1</text>"
    output="$output<text x=\"905\" y=\"30\" $textSettings>Part 2</text>"

    output="$output<text x=\"105\" y=\"60\" $textSettings>Average</text>"
    output="$output<text x=\"305\" y=\"60\" $textSettings>Best</text>"
    output="$output<text x=\"505\" y=\"60\" $textSettings>Worst</text>"

    output="$output<text x=\"705\" y=\"60\" $textSettings>Average</text>"
    output="$output<text x=\"905\" y=\"60\" $textSettings>Best</text>"
    output="$output<text x=\"1105\" y=\"60\" $textSettings>Worst</text>"

    for day in $(seq 1 "$cap"); do
        echo "$year - $day"
        y=$(echo $(($day*30+60))) 
        output="$output<text x=\"5\" y=\"$y\" $textSettings>Day $day:</text>"
        part1=$(ulimit -v 4000000 -t 100 && ./target/release/advent_of_code -s $year $day 1)
        part2=$(ulimit -v 4000000 -t 100 && ./target/release/advent_of_code -s $year $day 2)
        if [ -n "$part1" ]; then
            avg=$(echo "$part1" | jq .avg -r)
            best=$(echo "$part1" | jq .best -r)
            worst=$(echo "$part1" | jq .worst -r)
            if [ $? -eq 0 ]; then
                output="$output<text x=\"105\" y=\"$y\" $textSettings>$(emoji $avg) $avg</text>"
                output="$output<text x=\"305\" y=\"$y\" $textSettings>$(emoji $best) $best</text>"
                output="$output<text x=\"505\" y=\"$y\" $textSettings>$(emoji $worst) $worst</text>"
            else
                output="$output<text x=\"305\" y=\"$y\" fill=\"red\">🖨️❗</text>"
            fi
        fi
        if [ -n "$part2" ]; then
            avg=$(echo "$part2" | jq .avg -r)
            best=$(echo "$part2" | jq .best -r)
            worst=$(echo "$part2" | jq .worst -r)
            if [ $? -eq 0 ]; then
                output="$output<text x=\"705\" y=\"$y\" $textSettings>$(emoji $avg) $avg</text>"
                output="$output<text x=\"905\" y=\"$y\" $textSettings>$(emoji $best) $best</text>"
                output="$output<text x=\"1105\" y=\"$y\" $textSettings>$(emoji $worst) $worst</text>"
            else
                output="$output<text x=\"905\" y=\"$y\" fill=\"red\">🖨️❗</text>"
            fi
        fi
    done
    output="$output</svg>"
    echo "$output" > benchmarks-data/$year.svg
    site="$site<img src=\"https://loafey.se/advent_of_code/benchmarks/$year.svg\">"
done
site="$site </body></html>"
echo "$site" > index-temp.html