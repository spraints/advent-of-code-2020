import * as React from 'react'
import DayNG from './DayNG'

const BOARDING_PASS = /[FBLR]+/g

function parseDayInput(input: string): any {
  return input.match(BOARDING_PASS)
}

interface IOutputProps {
  output: IOutput
}

interface IOutput {
  max_id: number
  my_seat: number
}

function Output({output}: IOutputProps) {
  return (
    <div>
      <p>Max Seat ID: <b>{output.max_id}</b></p>
      <p>My Seat ID: <b>{output.my_seat}</b></p>
    </div>
  )
}

export default function() {
  return (
    <DayNG num="5" url="/api/day5" parseInput={parseDayInput}>
      {(output: IOutput) => <Output output={output}/>}
    </DayNG>
  )
}
