import { useState } from 'react';

const SUBMIT_URL = 'http://ec2-3-129-9-220.us-east-2.compute.amazonaws.com:3000/submit';

export default function useSubmitSolution() {
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [solution, setSolution] = useState(null);
  const [error, setError] = useState(null);

  const handleSubmit = async ({ code, ram, cpu, challenge_index = 1 }) => {
    setIsSubmitting(true);
    setError(null);
    setSolution(null);

    try {
      const requestBody = {
        constraints: { cpu: Number(cpu), ram: Number(ram) * 1000 },
        code: code,
        challenge_index: challenge_index
      };

      const response = await fetch(SUBMIT_URL, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(requestBody)
      });

      if (!response.ok) throw new Error(`Server responded with ${response.status}`);

      const data = await response.json();

      const soln = {
        //compiler data
        c_success: data.compiler.success,
        c_stdout: data.compiler.stdout,
        c_stderr: data.compiler.stderr,

        //runner data
        r_success: data.runner.success,
        r_stdout: data.runner.stdout,
        r_stderr: data.runner.stderr,

        //test case data
        runtime_us: data.runtime_us,
        test_cases: data.test_cases
      };

      setSolution(soln);
      return soln;

    } catch (err) {
      console.error('Submission error:', err);
      const fallback = {
        //compiler data
        c_success: false,
        c_stdout: "",
        c_stderr: "",

        //runner data
        r_success: false,
        r_stdout: "",
        r_stderr: "",

        //test case data
        runtime_us: 0,
        test_cases: []
      };
      setSolution(fallback);
      setError(err);
      return fallback;
    } finally {
      setIsSubmitting(false);
    }
  };

  return { handleSubmit, isSubmitting, solution, error };
}
