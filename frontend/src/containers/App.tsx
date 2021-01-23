import React from 'react';
import '../static/App.css';
import Home from "../pages/Home";
import Navbar from './Navbar';
import {
  BrowserRouter as Router,
  Switch,
  Route,
  useRouteMatch
} from "react-router-dom";
import Projects from '../pages/Projects';
import ContactMe from '../pages/ContactMe';
import Project from '../pages/Project';

function App() {
  return (
    <Router>
      <Navbar/>

        <div className="App">
          <Switch>
              <Route path="/projects">
                <Projects/>
              </Route>
              
              <Route path="/contact-me">
                <ContactMe/>
              </Route>

              <Route path="/project/:projectID">
                <Project/>
              </Route>

              <Route path="/">
                <Home/>
              </Route>
          </Switch>
        </div>
    </Router>
  );
}


export default App;
