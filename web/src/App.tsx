import * as React from 'react';

import Day from './Day';
import Day1 from './Day1';
import Day2 from './Day2';
import Day3 from './Day3';

class App extends React.Component<{}, { apiMessage: string }> {
  constructor(props: object) {
    super(props);

    this.state = { apiMessage: "Loading... (If this takes too long, the database might be down.)" };
  }

  public componentDidMount() {
    fetch("/api")
      .then(r => r.status === 500
        ? `(The server reported an error or cannot be reached. Is it compiling...?)`
        : r.text()
      )
      .then(apiMessage =>
        this.setState({
          apiMessage
        })
      );
  }

  public render() {
    return (
      <div className="p-3 container">
        <div className="jumbotron">
          <h1 className="header">
            Matt's Advent of Code 2020
          </h1>
        </div>

        <Day3/>

        <Day num="2">{(isPart2: boolean, input: string) =>
          <Day2 part2={isPart2} input={input}/>}</Day>

        <Day num="1">{(isPart2: boolean, input: string) =>
          <Day1 part2={isPart2} input={input}/>}</Day>

        <h2>Status</h2>
        <p>
          {this.state.apiMessage}
        </p>
      </div>
    );
  }
}

export default App;
