# Backend
Here is some prelimanary information about the backend and how it works.

## Compiling the C program
`compile.rs` contains the code that will compile the user's code submission. Each program will be compiled
with the user submitted code as a single file, and another file containing the main function that will parse
the command line parameters and run the function against a single test case. To be clear, each test case
means the program will be called with that test case as a command line argument, and then the main function
will parse that command line argument and call the user's function on it. The result of the user's function will
be appended to `test_cases_output.txt` so it may eventually be sent back to the frontend. The backend does not check
if the tests pass or not.

## The runner
If the program the user sends successfully compiles, the runner will be started. The runner is a Docker
container that has a bunch of flags predefined. Most notably, the workspace volume will be mounted so the
backend can easily access the runner's output.

## Communicating between the runner and backend
The backend will create a "client workspace" (`crate::client_workspace::ClienWorkspace`) for each
code submission. That directory will hold several things by the end of its runtime:

 + `user_code.c` user's submitted C code
 + `a.out` final compiled C program
 + `challenge.json` json challenge, will parse the test cases out of this
 + `test_cases_output.txt` text file containing 1 line per test case, each line just being the result of that test case
 + `runtime.txt` text file that contains only a single line representing runtime of the program in microseconds

## Challenges
Each challenge has a few requirements that are stored in `/app/challenges` as a json. The backend will find
and copy the proper challenge to the container in its workspace as `WORKSPACE/challenge.json`, and the runner
script will do the work of parsing the test cases out of it.