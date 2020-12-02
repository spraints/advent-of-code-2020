import * as React from 'react'
import IDayProps from './DayProps'

const NUMBERS = /\d+/g

interface IState {
  output: IOutput | null
}

interface IOutput {
  numbers: number[]
  result: number
}

class Day1 extends React.Component<IDayProps, IState> {
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
    return (
      <p>
        {this.state.output.numbers.join(' + ')} = 2020<br/>
        {this.state.output.numbers.join(' * ')} = <b>{this.state.output.result}</b>
      </p>
    )
  }

  private solve() {
    this.setState({output: null})
    const matches = this.props.input.match(NUMBERS)
    if (matches) {
      const numbers = [...matches].map((match: any) => parseInt(match, 10))
      const req = {
        body: JSON.stringify({
          numbers,
          part2: this.props.part2
        }),
        headers: {"Content-Type": "application/json"},
        method: "POST"
      }
      fetch("/api/day1", req)
        .then(r => r.json())
        .then((output: IOutput) => this.setState({output}))
    }
  }
}

export default Day1
