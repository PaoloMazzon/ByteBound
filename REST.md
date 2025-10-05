This is the source of truth for our REST apis. Please modify this if you ever
change the internal code handling the APIs.

```json
// submit POST
{
  "constraints": { 
  	"cpu": 500, // in mhz
  	"ram": 1000 // in bytes
  },
  "code": "...",
  "challenge_name": "challenge"
}

// submit response
{
  "compiled": true,
  "success": true,
  "runtime_us": 329814, // microseconds
  "errors": "":,
  "test_cases": [true, false, true, false, true]
}

// ai POST
{
  "prompt": "ai prompt"
}

// ai response
{
  "reply": ""
}
```
