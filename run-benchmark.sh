mkdir benchmarks-data

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

for year in "${YEARS[@]}"; do
    output="<svg height=\"830\" width=\"1300\" xmlns=\"http://www.w3.org/2000/svg\">"
    output="$output<rect width=\"100%\" height=\"100%\" fill=\"%230D1117\" />"
    output="$output<text x=\"10\" y=\"30\" fill=\"white\">Year $year</text>"
    output="$output<text x=\"305\" y=\"30\" fill=\"white\">Part 1</text>"
    output="$output<text x=\"905\" y=\"30\" fill=\"white\">Part 2</text>"

    output="$output<text x=\"105\" y=\"60\" fill=\"white\">Average</text>"
    output="$output<text x=\"305\" y=\"60\" fill=\"white\">Best</text>"
    output="$output<text x=\"505\" y=\"60\" fill=\"white\">Worst</text>"

    output="$output<text x=\"705\" y=\"60\" fill=\"white\">Average</text>"
    output="$output<text x=\"905\" y=\"60\" fill=\"white\">Best</text>"
    output="$output<text x=\"1105\" y=\"60\" fill=\"white\">Worst</text>"
    for day in $(seq 1 25); do
        echo "$year - $day"
        y=$(echo $(($day*30+60))) 
        output="$output<text x=\"5\" y=\"$y\" fill=\"white\">Day $day:</text>"
        part1=$(ulimit -v 4000000 -t 100 && ./target/release/advent_of_code -s $year $day 1)
        part2=$(ulimit -v 4000000 -t 100 && ./target/release/advent_of_code -s $year $day 2)
        if [ -n "$part1" ]; then
            output="$output<text x=\"105\" y=\"$y\" fill=\"white\">$(echo "$part1" | jq .avg -r)</text>"
            output="$output<text x=\"305\" y=\"$y\" fill=\"white\">$(echo "$part1" | jq .worst -r)</text>"
            output="$output<text x=\"505\" y=\"$y\" fill=\"white\">$(echo "$part1" | jq .best -r)</text>"
        else
            output="$output<text x=\"105\" y=\"$y\" fill=\"red\">Missing, timeout or OOM!</text>"
        fi
        if [ -n "$part2" ]; then
            output="$output<text x=\"705\" y=\"$y\" fill=\"white\">$(echo "$part2" | jq .avg -r)</text>"
            output="$output<text x=\"905\" y=\"$y\" fill=\"white\">$(echo "$part2" | jq .worst -r)</text>"
            output="$output<text x=\"1105\" y=\"$y\" fill=\"white\">$(echo "$part2" | jq .best -r)</text>"
        else 
            output="$output<text x=\"705\" y=\"$y\" fill=\"red\">Missing, timeout or OOM!</text>"
        fi
    done
    output="$output</svg>"
    echo "$output" > benchmarks-data/$year.svg
    site="$site<img src=\"https://loafey.se/advent_of_code/benchmarks/$year.svg\">"
done
site="$site</body></html>"
echo "$site" > index-temp.html