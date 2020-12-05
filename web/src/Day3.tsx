import * as React from 'react'
import DayNG from './DayNG'

function parseDay3Input(input: string): any {
  return input
}

interface IOutputProps {
  output: IOutput
}

interface IOutput {
  product: number
  runs: IRun[]
}

interface IRun {
  slope: number[]
  collisions: number
  rendered: string
}

function Day3Output({output}: IOutputProps) {
  return (
    <div>
      <p>Result: <b>{output.product}</b></p>
      {output.runs.map(renderRun)}
    </div>
  )
}

function renderRun(run: IRun) {
  return (
    <div>
      <p>Slope: ({run.slope.join(", ")})</p>
      <pre>{run.rendered}</pre>
    </div>
  )
}

export default function() {
  return (
    <DayNG num="3" url="/api/day3" parseInput={parseDay3Input}>
      {(output: any) => <Day3Output output={output}/>}
    </DayNG>
  )
}
