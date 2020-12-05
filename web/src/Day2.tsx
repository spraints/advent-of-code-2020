import * as React from 'react'
import DayNG from './DayNG'

const PASSWORD_LINE = /(\d+)-(\d+) (.): (.*)/g
const PASSWORD_LINE2 = /(\d+)-(\d+) (.): (.*)/

function parseDay2Input(input: string): any {
  const matches = input.match(PASSWORD_LINE)
  return [...matches!].map((match: string) => {
    const m = match.match(PASSWORD_LINE2)
    if (!m) {
      return {}
    }
    return {
      password: m[4],
      policy: {min: parseInt(m[1], 10), max: parseInt(m[2], 10), c: m[3]}
    }
  })
}

interface IOutputProps {
  output: IOutput
}

interface IOutput {
  valid: IPasswordLine[]
  invalid: IPasswordLine[]
}

interface IPasswordLine {
  policy: IPasswordPolicy
  password: string
}

interface IPasswordPolicy {
  min: number
  max: number
  c: string
}

function Day2Output({output}: IOutputProps) {
  const Res = (props: any) => (
    <div>
      <i>{props.label}</i>
      <ul>
        {props.items.map((line: IPasswordLine) => (<li>{line.policy.min}-{line.policy.max} {line.policy.c}: {line.password}</li>))}
      </ul>
    </div>
  )
  return (
    <p>
      <b>{output.valid.length}</b>
      <Res label="Valid" items={output.valid}/>
      <Res label="Not valid" items={output.invalid}/>
    </p>
  )
}

export default function() {
  return (
    <DayNG num="2" url="/api/day2" parseInput={parseDay2Input}>
      {(output: any) => <Day2Output output={output}/>}
    </DayNG>
  )
}
