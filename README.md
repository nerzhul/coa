# Container orchestrator insights

This is an experimental project around container orchestrators

# Architecture

Project is composed of multiple components:
* API: expose a OpenAPI REST API to put or retrieve insights
* Analyzer: one or multiple analyzer retrieving some flow and publishing to API
* Frontend: a web frontend to display insights (not yet implemented)
* Database: a PostgreSQL backed database (MySQL and variantes won't be supported).

Goal is to have multiple analyzers, some directly provided in the project, some provided by the
community. Connected together it offer end user insights about the global health of their platform.

# Insights

* Enhancement proposals
* Issues/Defects
* Costs

## Enhancements proposals

It covers all ideas permitting to enhance reliability, security or other topics.

## Issues

It reports configuration problems or object state problems, or any other problem.

### Categories

Here is the current list of categories:

* Configuration
* Security
* Reliability
* Performance

# First milestone target

* Store issues in database and retrieve them in end user model
* Create a first analyzer to detect issues
	* Connect to Kubernetes API
	* Streamline objects/events
	* Apply analysis objects in order to create insights
	* Store insights in API
