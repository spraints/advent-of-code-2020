import * as React from 'react'
import DayNG from './DayNG'

const NUMBERS = /\d+/g

function parseDay1Input(input: string): any {
  const matches = input.match(NUMBERS)
  return [...matches!].map((match: any) => parseInt(match, 10))
}

interface IOutputProps {
  output: IOutput
}

interface IOutput {
  numbers: number[]
  result: number
}

function Day1Output({output}: IOutputProps) {
  return (
    <p>
      {output.numbers.join(' + ')} = 2020<br/>
      {output.numbers.join(' * ')} = <b>{output.result}</b>
    </p>
  )
}

export default function() {
  return (
    <DayNG num="1" url="/api/day1" parseInput={parseDay1Input}>
      {(output: any) => <Day1Output output={output}/>}
    </DayNG>
  )
}
