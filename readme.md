# Earwig Template Engine/Preprocessor 0.3.0

**The python preprocessor for the Earwig web framework.**

__NOTE__ : This pre-processor is expieremental and is no way reccomended or required in order to use the earwig web framework.  This was more of an expierement I made to see what I could use a pre-processor for in my web framework.

# Templates

__Templates are keywords in the template engine that are built into the engine.  If a template has the `?` symbol behind it, then it is renderable/executeable.  Templates with a star infront of them render to python and should only be used at an indentation level of 0 (Best practices are probably to avoid using these templates in general).__

## ?HEADERS *

 - **Generates earwig python to set the response Headers.**

```
?HEADERS:
	Username: "name"
	Password: "password123"
	Data:
		subData: "123"
		otherSubData: "456"
```

## ?MIME *

 - **Generates earwig python to set the response MIME type.**

```
?MIME: JSON
```

## ?PRESET

 - **Used to create or render presets.**

```
?PRESET:
	API:
```

### NEW_PRESETS

 - **Sub-template used to declare new presets.**

```
?PRESET:
	NEW_PRESETS:
		API:
			?HEADERS:
				Cors: "*"
				Auth:
					Token: "*&^87^*&^8^$%643$%3%"
					ID: 1
			?MIME: JSON
		HTML:
			?MIME: HTML
			?INSERT:
				PATH: "../PAGES/navbar.ear"
				SUBSTITUTIONS:
					#Home is set to true because it is the active tab in the navbar.
					Home: "True"
```

## ?INSERT

 - **Used to insert other .ear or .py files into the position at the template.**

### PATH

 - **Sub-template used to declare the path to the file to insert.**

### SUBSTITUTIONS

 - **Sub-template used to declare substitutions to swap out.**

```
?INSERT:
	PATH: "../PAGES/navbar.ear"
	SUBSTITUTIONS:
		#Home is set to true because it is the active tab in the navbar.
		Home: "True"
```

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
