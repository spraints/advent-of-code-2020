import * as React from 'react'
import IDayProps from './DayProps'

const PASSWORD_LINE = /(\d+)-(\d+) (.): (.*)/g
const PASSWORD_LINE2 = /(\d+)-(\d+) (.): (.*)/

interface IState {
  output: IOutput | null
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

class Today extends React.Component<IDayProps, IState> {
  constructor(props: IDayProps) {
    super(props)
    this.state = { output: null }
    this.solve()
  }

  public componentDidUpdate(prevProps: IDayProps) {
    if (this.props.input !== prevProps.input || this.props.part2 !== prevProps.part2) {
      this.solve()
    }
  }

  public render() {
    if (this.state.output == null) {
      return ''
    }
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
        <b>{this.state.output.valid.length}</b>
        <Res label="Valid" items={this.state.output.valid}/>
        <Res label="Not valid" items={this.state.output.invalid}/>
      </p>
    )
  }

  private async solve() {
    this.setState({output: null})
    const matches = this.props.input.match(PASSWORD_LINE)
    if (matches) {
      const lines = [...matches].map((match: string) => {
        const m = match.match(PASSWORD_LINE2)
        if (!m) {
          return {}
        }
        return {
          password: m[4],
          policy: {min: parseInt(m[1], 10), max: parseInt(m[2], 10), c: m[3]}
        }
      })

      // tslint:disable:no-console
      // console.log(passwords)

      const req = {
        body: JSON.stringify({lines, part2: this.props.part2}),
        headers: {"Content-Type": "application/json"},
        method: "POST"
      }
      const res = await fetch("/api/day2", req)
      if (res.status === 200) {
        const output = await res.json() as IOutput
        this.setState({output})
      }
    }
  }
}

export default Today
