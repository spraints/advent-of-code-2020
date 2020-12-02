import * as React from 'react';

import Day from './Day';
import Day1 from './Day1';
import Day2 from './Day2';

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

        <Day num="2"><Day2/></Day>
        <Day num="1"><Day1/></Day>

        <h2>Status</h2>
        <p>
          {this.state.apiMessage}
        </p>
      </div>
    );
  }
}

export default App;
