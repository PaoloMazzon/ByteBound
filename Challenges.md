# Challenges Structure
Each challenge should be structured in a specific way in a json file in 
`challenges/challenge_x.json`. Please refer to the below sample.

```json5
{
  "problem": {
    "title": "Fibonacci Climbing Stairs",
    "description": "You are climbing a staircase with n steps. Each time you can either climb 1 step or 2 steps.",
    "question": "In how many distinct ways can you climb to the top?"
  },
  "examples": [
    {
      "input": "n = 2",
      "output": "2",
      "explanation": "Two ways to climb: (1+1) or (2)"
    },
    {
      "input": "n = 3",
      "output": "3",
      "explanation": "Three ways to climb: (1+1+1), (1+2), or (2+1)"
    },
    {
      "input": "n = 5",
      "output": "8",
      "explanation": "Eight different combinations of steps"
    }
  ],
  "test_cases": [
    {
      "input": 8,
      "output": 21
    },
    {
      "input": 10,
      "output": 55
    },
    {
      "input": 12,
      "output": 144
    },
    {
      "input": 36,
      "output": 14930352
    }
  ],
  "difficulty": "Medium",
  "topics": ["Dynamic Programming", "Fibonacci Sequence", "Recursion", "Optimization"]
}
```

Each challenge should also have a corresponding `sample_x.c` file and `main_x.c` file that correspond
to the challenge's sample code and the main code it will be linked against.