import * as React from 'react'

import './Day.css'

interface IProps {
  num: string
}

interface IState {
  visible: boolean
}

class Day extends React.Component<IProps, IState> {
  constructor(props: IProps) {
    super(props)

    this.state = { visible: false }

    this.show = this.show.bind(this)
    this.hide = this.hide.bind(this)
  }

  public render() {
    if (!this.state.visible) {
      return (
        <div className="Day-row Day-row-collapsed container">
          <div className="row" onClick={this.show}>
            <h2>Day {this.props.num}</h2>
          </div>
        </div>
      )
    }
    return (
      <div className="Day-row Day-row-expanded container">
        <div className="row" onClick={this.hide}>
          <h2>Day {this.props.num}</h2>
        </div>
        <div className="row">
          (TODO - form for today's input)
        </div>
      </div>
    )
  }

  private show() {
    this.setState({visible: true})
  }

  private hide() {
    this.setState({visible: false})
  }
}

export default Day
