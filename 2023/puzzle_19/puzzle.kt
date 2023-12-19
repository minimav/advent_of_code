import java.io.*

fun readFile(fileName: String): String 
  = File(fileName).readText(Charsets.UTF_8)


fun buildRuleFunc(rule: String): (Map<String, Int>) -> String {
  if (rule.contains(":")) {
    val (condition, result) = rule.split(":")
    val key: String = condition[0].toString()
    val op: String = condition[1].toString()
    val value: Int = condition.drop(2).toInt()

    if (op == ">") {
      return { it -> if (it[key]!! > value) result else "" }
    } else if (op == "<") {
      return { it -> if (it[key]!! < value) result else "" }
    } else {
      // Should not occur
      return { it -> if (it[key]!! == value) result else "" }
    } 
  }
  // R, A and other id cases, always true
  return { _ -> rule }
}

fun buildRuleFuncs(rules: List<String>): List<(Map<String, Int>) -> String> {
  return rules.map { buildRuleFunc(it) }
}

data class RuleInfo(val key: String, val op: String, val value: Int, val to: String)


fun buildRuleInfo(rule: String): RuleInfo {
  if (rule.contains(":")) {
    val (condition, result) = rule.split(":")
    val key: String = condition[0].toString()
    val op: String = condition[1].toString()
    val value: Int = condition.drop(2).toInt()
    return RuleInfo(key, op, value, result)
  }
  // R, A and other id cases, always true
  return RuleInfo(rule, "", 0, "")
}


fun part1(rawInput: String) {
  // Parse functions e.g. rfg{s<537:gd,x>2440:R,A}
  val lines = rawInput.split("\n")
  val workflows = lines.filter { !(it.startsWith("{")) && it.isNotBlank() }.map {
    val (id, remainder) = it.split("{")
    val rawRules = remainder.dropLast(1).split(",")
    val rules = buildRuleFuncs(rawRules)
    id to rules
  }.toMap()

  // Parse inputs e.g. {x=787,m=2655,a=1222,s=2876}
  val inputData = lines.filter { it.startsWith("{") }.map {
    it.removeSurrounding("{", "}").split(",").map {
      it.split("=").let { (key, value) -> key to value.toInt() }
    }.toMap()
  }

  var answer = 0  
  for (input in inputData) {
    var currentKey = "in"
    var shouldBreakFully = false
    while (true) {
      if (shouldBreakFully) {
        break
      }

      var workflow = workflows[currentKey]!!
      // Apply rules in workflow in order until one is satisfied, there should
      // always be at least one
      for (rule in workflow) {
        var result = rule(input)
        if (result == "") {
          continue
        } else if (result == "R") {
          shouldBreakFully = true
          break
        } else if (result == "A") {
          answer += input.values.sum()
          shouldBreakFully = true
          break
        } else {
          currentKey = result
          break
        }
      }
    }
  }
  println(answer)
}


fun getRangedInputSize(input: Map<String, IntRange>): Long {
  // Range is inclusive, so 1..1 should have size 1
  // Need to use Long to avoid overflow
  return input.values.map { (it.last - it.first + 1).toLong() }.reduce {
    acc, i -> acc * i
  }
}

fun split(range: IntRange, value: Int, op: String): Pair<IntRange, IntRange> {
  // Deal with no split cases
  if (value > range.last) {
    return Pair(range, 1..0)
  } else if (value < range.first) {
    return Pair(1..0, range)
  }

  // value is within the range
  val leftRange = if (op == ">") range.first..value else range.first..(value - 1)
  val rightRange = if (op == ">") (value + 1)..range.last else value..range.last
  return Pair(leftRange, rightRange)
}

fun part2(rawInput: String) {
  // Parse functions e.g. rfg{s<537:gd,x>2440:R,A}
  val lines = rawInput.split("\n")
  val workflows = lines.filter { !(it.startsWith("{")) && it.isNotBlank() }.map {
    val (id, remainder) = it.split("{")
    val rawRules = remainder.dropLast(1).split(",")
    val rules = rawRules.map { buildRuleInfo(it) }
    id to rules
  }.toMap()

  // Store input range, workflow key and rule index within workflow
  // Storing the latter simplifies keeping track of ranges since we can treat
  // each rule within a workflow as a separate step
  var rangeInputs: MutableList<Triple<Map<String, IntRange>, String, Int>> = mutableListOf(
    Triple(mapOf("x" to 1..4000, "m" to 1..4000, "a" to 1..4000, "s" to 1..4000), "in", 0)
  )
  
  var answer: Long = 0 
  while (rangeInputs.size > 0) {
    val (input, workflowKey, ruleIndex) = rangeInputs.removeAt(0)
    
    val workflow = workflows[workflowKey]!!
    val rule = workflow[ruleIndex]
    
    if (rule.op == ">" || rule.op == "<") {

      val range = input[rule.key]!!

      // Split into true and false ranges
      val falseRange: IntRange
      val trueRange: IntRange
      val ranges = split(range, rule.value, rule.op)

      if (rule.op == ">") {
        falseRange = ranges.first
        trueRange = ranges.second
      } else {
        trueRange = ranges.first
        falseRange = ranges.second
      }
      if (!falseRange.isEmpty()) {
        // Pass false range to next rule in same workflow
        val inputWithFalseRange = input.toMutableMap()
        inputWithFalseRange[rule.key] = falseRange
        rangeInputs.add(Triple(inputWithFalseRange, workflowKey, ruleIndex + 1))
      }
      
      if (!trueRange.isEmpty()) {
        // Determine what to do with the true range based on condition
        val inputWithTrueRange = input.toMutableMap()
        inputWithTrueRange[rule.key] = trueRange
        if (rule.to == "R") {
          // Reject the true range
          continue
        } else if (rule.to == "A") {
          // True range is accepted, so add to answer
          answer += getRangedInputSize(inputWithTrueRange)
          continue
        } else {
          // Redirect to another workflow
          rangeInputs.add(Triple(inputWithTrueRange, rule.to, 0))
        }
      }
    } else if (rule.key == "A") {
      // Accept the range, so add all contained parameter combinations to answer
      answer += getRangedInputSize(input)
      continue
    } else if (rule.key == "R") {
      // Reject the range
      continue
    } else {
      // Move to another workflow as rule is just a redirection
      rangeInputs.add(Triple(input, rule.key, 0))
    }
  }
  println(answer)
}

fun main(args : Array<String>) {
    part1(readFile("puzzle_19/example.txt"))
    part1(readFile("puzzle_19/input.txt"))

    part2(readFile("puzzle_19/example.txt"))
    part2(readFile("puzzle_19/input.txt"))
}