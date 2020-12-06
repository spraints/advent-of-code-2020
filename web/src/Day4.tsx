import * as React from 'react'
import Day from './Day'

function parseDayInput(input: string): any {
  return input
}

interface IOutputProps {
  output: IOutput
}

interface IOutput {
  valid: number
  failures: string[]
}

function Output({output}: IOutputProps) {
  return (
    <div>
      <p><b>{output.valid}</b></p>
      <p>problems:</p>
      <ul>
        {output.failures.map(res => (<li><pre>{res}</pre></li>))}
      </ul>
    </div>
  )
}

export default function() {
  return (
    <Day num="4" url="/api/day4" parseInput={parseDayInput}>
      {(output: IOutput) => <Output output={output}/>}
    </Day>
  )
}
