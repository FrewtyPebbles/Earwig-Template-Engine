# Earwig Template Engine/Preprocessor 0.3.0

**The python preprocessor for the Earwig web framework.**

# Templates

__Templates are keywords in the template engine that are built into the engine.  If a template has the `?` symbol behind it, then it is renderable/executeable__

## ?HEADERS

 - **Generates earwig python to set the response Headers.**

## ?MIME

 - **Generates earwig python to set the response MIME type.**

## ?PRESET

 - **Used to create or render presets.**

### NEW_PRESETS

 - **Sub-template used to declare new presets.**

## ?REQUEST_LIMIT

 - **Generates the earwig python to set the request limit.**

## ?INSERT

 - **Used to insert other .ear or .py files into the position at the template.**

### PATH

 - **Sub-template used to declare the path to the file to insert.**

### SUBSTITUTIONS

 - **Sub-template used to declare substitutions to swap out.**

# Not implemented yet

## ?REQUEST

 - Different file types will be handled differently

```
?REQUEST:
	URL: https://api.com/authhandler.py
	BODY:
		USERNAME: "root"
		PASSWORD: "password"
```

 - **.py files will concatenate the file into the .ear file, much like an INSERT or PRESET**

```
?REQUEST:
	URL: https://api.com/page.ear
	SUBSTITUTIONS:
		name: "username"
	BODY:
		USERNAME: "root"
		PASSWORD: "password"
```

 - **.ear files will allow for the same substitutions that INSERT allows for**
