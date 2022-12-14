# Logbook

## Development

### Roadmap

#### Milestone 1

- [ ] Able to see a list of my unique tags
  - Create a sub-command architecture
  - Consider moving away from the shortened form of the command and consider
    leaving those duties to aliases and shell functions.
  - SubCommands
    - list | show | ls
      - Query language
    - log | _
      - amend / append / edit
    - tags 
      - Show tags and tag stats and editor (eventually)
  - Probably want to spend some time thinking through all the different flags and
    how queries will be formulated
- [ ] Able to filter inclusive/exclusive by tags
- [ ] Support a config file
- [ ] Parameterize data file location and other variables
- [ ] -e, --editor <editor>
  - Defaults to $VISUAL then $EDITOR
  - How do you open vim (or any editor) such that onSave + quit the contents of
    the buffer are immedietly fed into lgbk?

#### Milestone 2

- [ ] Smart printing based on user's terminal width
- [ ] Query based on date-times and ranges
- [ ] Able to pass arbitrary strftime format string
- [ ] Alternative storage formats (binary, plaintext)
  - Check the crate bincode
- [ ] Ability to limit the length of the entry
- [ ] Can filter tags out of entry text

#### Milestone 3

- [ ] Support natural language dates and times
- [ ] _Ad Hoc_ tag mgmt 
- [ ] Advanced output formats (json, poreclin, csv)
- [ ] Accept stdin for integration with other tools

#### Milestone 4

- [ ] Embed Lua to facilitate callbacks
  - OnSubmit, onParse, onSave... idk ***Events***
  - Could be used to check for spelling errors or similar tag
  - Could even strip entries of their tags before saving
- [ ] Experiment with macOS native development by wrapping the cli in a Swift app
- [ ] Create a vim plugin
