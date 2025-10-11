This is the source of truth for our REST apis. Please modify this if you ever
change the internal code handling the APIs.

```json5
// POST /submit endpoint
{
  "constraints": { 
  	"cpu": 500, // in mhz
  	"ram": 1000000 // in bytes
  },
  "code": "...", // c code to be compiled
  "challenge_index": 0
}

// and the /submit reply:
{
  // information on the compilation of the program
  "compiler" {
    "success": true, // pass/fail
    "stdout": "",
    "stderr": ""
  },
  // information on the running of the program (this is invalid if it didn't compile)
  "runner" {
    "success": true, // did the program crash in anyway
    "stdout": "",
    "stderr": "",
  },
  // information on each individual test case
  "test_cases": [
    {
      "runtime_us": 98321,
      "expected_value": 12,
      "actual_value": 12,
      "input_value": 5,
      "memory_usage_kb": 42356
    },
    {
      "runtime_us": 48321,
      "expected_value": 12,
      "actual_value": 12,
      "input_value": 5,
      "memory_usage_kb": 42356
    },
  ]
}

// /challenge_query (used to get data about challenges available)
{
  // no payload yet
}

// /challenge_query reply
{
  "count": 3,
  "challenges": [
    {
      "name": "xyz",
      "difficulty": "easy/medium/hard/expert",
      "brief": "brief description"
    },
    {
      // ...
    }
  ]
}

// /challenge_info_query (used to get info about a specific challenge)
{
  "index": 0, // index into the previously gotten challenge list from /challenge_query
}

// and the reply
{
  "name": "xyz",
  "difficulty": "easy/medium/hard/expert",
  "brief": "brief description",
  "description": "longer description in markdown",
  "sample_code": "this code should be in the editor by default"
}
```
