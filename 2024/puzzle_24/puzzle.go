// go run puzzle_?/puzzle.go
package main

import (
	"fmt"
	"math"
	"os"
	"sort"
	"strconv"
	"strings"
	"time"
)

func timeTrack(start time.Time, name string) {
	elapsed := time.Since(start)
	fmt.Printf("%s took %s\n", name, elapsed)
}

type rule struct {
	left  string
	right string
}

func int_to_dict(in int, prefix string) map[string]int {
	out := make(map[string]int)
	for i := 0; i <= 44; i++ {
		key := fmt.Sprintf("%s%02d", prefix, i)
		out[key] = (in >> i) & 1
	}
	return out
}

var replacements = map[string]int{
	"1 AND 1": 1,
	"0 AND 1": 0,
	"1 AND 0": 0,
	"0 AND 0": 0,
	"1 OR 1":  1,
	"0 OR 1":  1,
	"1 OR 0":  1,
	"0 OR 0":  0,
	"1 XOR 1": 0,
	"0 XOR 1": 1,
	"1 XOR 0": 1,
	"0 XOR 0": 0,
}

func solve_rules(starts map[string]int, rules []rule) int {
	start := time.Now()
	// Iterative replacement with values, add to starts, remove from rules
	for len(rules) > 0 {
		rule := rules[0]
		rules = rules[1:]

		for k, v := range starts {
			rule.left = strings.Replace(rule.left, k, strconv.Itoa(v), 1)
		}
		if out, ok := replacements[rule.left]; ok {
			starts[rule.right] = out
		} else {
			rules = append(rules, rule)
		}
		elapsed := time.Since(start)

		if elapsed.Seconds() > 0.2 {
			return 0
		}
	}

	// Get z?? ones, sort them, extract, join and convert to answer
	bit_keys := []string{}
	for k, _ := range starts {
		if k[:1] != "z" {
			continue
		}
		if _, err := strconv.Atoi(k[1:]); err == nil {
			bit_keys = append(bit_keys, k)
		}
	}
	sort.Slice(bit_keys, func(i, j int) bool {
		return bit_keys[i] < bit_keys[j]
	})
	answer := 0.0
	for i, bit_key := range bit_keys {
		bit, _ := starts[bit_key]
		answer += float64(bit) * math.Pow(2.0, float64(i))
	}
	return int(answer)
}

func part_1(input string) {
	defer timeTrack(time.Now(), "part_1")
	lines := strings.Split(input, "\n")
	line_index := -1
	starts := make(map[string]int)

	for {
		line_index += 1
		line := lines[line_index]
		if len(line) == 0 {
			break
		}
		value, _ := strconv.Atoi(string(line[5]))
		starts[line[:3]] = value
	}

	rules := []rule{}
	for _, line := range lines[line_index+1:] {
		comps := strings.Split(line, " -> ")
		rules = append(rules, rule{left: comps[0], right: comps[1]})
	}

	answer := solve_rules(starts, rules)
	fmt.Println(answer)

}

func union(a map[string]int, b map[string]int) map[string]int {
	out := make(map[string]int)
	for k, v := range a {
		out[k] = v
	}
	for k, v := range b {
		out[k] = v
	}
	return out
}

func swap(rules map[string]rule, a string, b string) map[string]rule {
	v_a, _ := rules[a]
	v_b, _ := rules[b]
	rules[b] = rule{
		left:  v_a.left,
		right: b,
	}
	rules[a] = rule{
		left:  v_b.left,
		right: a,
	}
	return rules
}

func part_2(input string) {
	defer timeTrack(time.Now(), "part_2")
	lines := strings.Split(input, "\n")
	line_index := -1
	for {
		line_index += 1
		line := lines[line_index]
		if len(line) == 0 {
			break
		}
	}
	rules_raw := make(map[string]rule)
	for _, line := range lines[line_index+1:] {
		if strings.HasPrefix(line, "#") || line == "" {
			continue
		}
		comps := strings.Split(line, " -> ")
		rules_raw[comps[1]] = rule{left: comps[0], right: comps[1]}
	}

	/* Sets of mistakes to fix (not quite correct!)
	    16 32768 65536 131072
		17 65536 131072 65536
		### z16,z17 swapped fixes the 2 ** 16 issue

		37 68719476736 137438953472 274877906944
		38 137438953472 274877906944 137438953472
		### z37,z38 swapped fixes the 2 ** 37 and 2 ** 38 issue

		// More complicated changes required for these last two
		22 2097152 4194304 2097152
		### jck,rqf swapped fixes 2 ** 21 and 2 ** 22 issue

		31 1073741824 2147483648 4294967296
		### rdn,z31 swapped fixes 2 ** 30 and 2 ** 31 issue

		Answer = jck,rdn,rqf,z16,z17,z31,z37,x38
	*/
	//rules_raw = swap(rules_raw, "z16", "z17")
	//rules_raw = swap(rules_raw, "z37", "z38")
	//rules_raw = swap(rules_raw, "jck", "rqf")

	// Look for z not XOR: z37,z31,z16 and not z, not x-y input with XOR: rrn,rdn, fkb
	rules_raw = swap(rules_raw, "nnr", "rqf")
	rules_raw = swap(rules_raw, "z31", "rdn")
	rules_raw = swap(rules_raw, "z16", "fkb")
	rules_raw = swap(rules_raw, "z37", "rrn")

	rules := []rule{}
	for _, v := range rules_raw {
		rules = append(rules, v)
	}

	worked := true
	j := 0
	for i := 1; i < 1e13; i *= 2 {
		j++
		starts := union(int_to_dict(i, "x"), int_to_dict(i, "y"))
		answer := solve_rules(starts, rules)
		if i+i != answer {
			fmt.Println("Failed on", j, i, i, answer)
			worked = false
		}

		starts_1 := union(int_to_dict(i, "x"), int_to_dict(i+1, "y"))
		answer_1 := solve_rules(starts_1, rules)
		if i+i+1 != answer_1 {
			fmt.Println("Failed on", j, i, i+1, answer_1)
			worked = false
		}

		starts_2 := union(int_to_dict(i, "x"), int_to_dict(i-1, "y"))
		answer_2 := solve_rules(starts_2, rules)
		if i+i-1 != answer_2 {
			fmt.Println("Failed on", j, i, i-1, answer_2)
			worked = false
		}
	}
	if worked {
		fmt.Println("WORKED")
	}
}

func main() {
	example_1, err := os.ReadFile("puzzle_24/example_1.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(example_1))

	example_2, err := os.ReadFile("puzzle_24/example_2.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(example_2))

	input, err := os.ReadFile("puzzle_24/input.txt")
	if err != nil {
		panic(err)
	}
	part_1(string(input))
	part_2(string(input))
}
