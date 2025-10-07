This is the source of truth for our REST apis. Please modify this if you ever
change the internal code handling the APIs.

```json5
// POST /submit endpoint
{
  "constraints": { 
  	"cpu": 500, // in mhz
  	"ram": 1000 // in bytes
  },
  "code": "...", // c code to be compiled
  "challenge_name": "challenge"
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
    "runtime_us": 329814, // program runtime in microseconds
    "stdout": "",
    "stderr": "",
  },
  "test_cases": [true, false, true, false, true] // whether or not each test case passed
}

// POST /ai endpoint
{
  "prompt": "ai prompt"
}

// and the /ai reply:
{
  "reply": ""
}
```
