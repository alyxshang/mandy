# MANDY SITES :sparkles:

Mandy is a static-site generator. Mandy takes files ending in the `.markdown` file extension containing some content written in the Markdown language and compiles these files into HTML files. The content is styled via CSS obtained from files written in the SASS language. The generated HTML's structure is dictated by a set of layouts written in the Liquid language. To apply correct routing for your environment, the `MANDY_ENV` environment variable needs to be set to either `production` or `development`.

Every Mandy project is a collection of files and folders telling Mandy how to compile these files and folders into a static site. This collection of files and folders is called a Mandy project. Once theses files and folders are parsed and read by Mandy, Mandy produces a set of HTML files in a sub-directory of the Mandy project. The files in this sub-directory are called a "compiled Mandy project". For the sake of this explanation, one shall assume that there is a directory called "mysite" containing a Mandy project. Every Mandy project features the files and folders outlined in the following sections.

## Configuration

The configuration file can be writen in either the JSON format or the YAML format and has some compulsory options and some user-specifiable options and has to be called either `config.yml` or `config.json`. The configuration file is stored at the root of the Mandy project. These options are as follows:

- `title`: This option specifies the name of your Mandy project.
- `description`: This option describes your Mandy project.
- `prod_url`: This option specifies the root URL when your compiled Mandy project is being served in a production environment.
- `dev_url`: This option specifies the root URL when your compiled Mandy project is being served in a development environment.
- `tl_domain`: This option specifies the top-level domain under which your compiled Mandy project will be served. The contents of this option is only used when you enable SEO for your Mandy project.
- `seo`: This option specifies whether to enable SEO for your Mandy project or not.
- `dist_dir`: This option specifies in which sub-directory the files for your compiled Mandy project should be placed.
- `copy_files`: This option specifies whether Mandy should copy files and directories from your Mandy project to the sub-directory containing your compiled Mandy project.
- `has_loop_content`: This option specifies whether there is content that is iterative in some respect in your Mandy project.
- `loop_content_dirs`: This option specifies which directories in your Mandy poject contain iterative content.
- `copy_entities`: This option specifies which directories and files in your Mandy poject should be copied to the sub-directory containing your compiled Mandy project.
- `user_config`: This option lets you specify any number of key-value pairs for any extra information you may want to specify in your configuration file.

## Layouts and templating

Layouts are stored in the `layouts` directory at the root of your Mandy project. These layouts are called upon in the frontmatter of a file containing content written in the Markdown language via the `layout` option. To make the managing and writing of layouts easier, Mandy forces the user to use partial templates, also written in the Liquid templating language. Layouts are called only via their names (not including the file extension or any directories included in the name.) To render content using these layouts, the following namespaces with the following variables exist and are available in every Mandy project.

- `site`: These variables are taken from the configuration options specified in the project's configuration file. User-specified variables can be called upon via the `site.user_config` namespace. The snippet of code below illustrates how to call upon data in this namespace in a Liquid template.

```Liquid
<h1>{{ site.title }}</h1>
<h2>{{ site.user_config.tagline }}</h2>
```

- `page`: These variables are taken from the frontmatter options of a file containing Markdown code and ending in the `.markdown` extension. The namespace `page.params` allows access to user-specified variables from a content file's frontmatter. The variable `page.content` allows access to the content in HTML format of a file written in the Markdown format. The snippet of code below illustrates how to call upon data in this namespace inside a Liquid template.

```Liquid
<h2>{{ page.params.title }}</h2>
<p>{{ page.content }}</p>
```

- `data`: If a data directory exists containing files with data, this data can be accessed through the `data["key"]` variable, where `key` represents the name of the file without the file extension. The data inside each file can be looped over with Liquid's `for` keyword. The example below features social-media links being looped over from the file `data/socials.yml`.

```Liquid
{% for item in data["socials"] %}
 <p><a href="{{ item.link }}">{{ item.name }}</a></p>
{% endfor %}
```

- `loop_content`: If the Mandy project contains iterative content and access to this content has been enabled in the configuration file, the content from this iterative content can be accessed via the `loop_content["key"]` variable, where `key` is a representation for the directory in which the iterative content is stored. This directory also needs to be registered in the project's configuration file. The snippet of code below illustrates how to call upon data in this namespace inside a Liquid template. The example features posts from the `posts` directory being looped over.

```Liquid
{% for post in loop_content["posts"] %}
 <h2>{{ post.title }}</h2>
 <p>{{ post.description }}</p>
 <p><a href="{{ baseurl }}{{ post.url }}">Read Post</a></p>
{% endfor %}
```

Partial templates are stored in the `partials` directory at the root of every Mandy project.

## Styling templates via SASS

Styling HTML templates can be done via normal CSS or by using SASS. If you do use SASS in your project, the point of entry for your SASS code must be a file called `ìndex.scss` located inside the `sass` directory. This directory must be at the root of your Mandy project for your SASS code to be detected and compiled.

## Site Data

Site data is stored in a directory called `data` at the root of a Mandy project. The files in this directory can be either of the YAML format or the JSON format. In either case, an array of maps must be present in each file. Having data in a Mandy project is entirely optional. If, however, Mandy detects this directory, the data inside it will be made available for use in templates.

```YAML
# data/socials.yml
- name: GitHub
  link: https://github.com/alyxshang
- name: Instagram
  link": https://instagram.com/alyxshang
```

The same data stored in JSON format may look like this:

```JSON
// data/socials.json
[
  {
    "name": "GitHub",
    "link": "https://github.com/alyxshang"
  },
  {
    "name": "Instagram",
    "link": "https://instagram.com/alyxshang"
  }
]
```

## Content Files

Content in a Mandy project is stored in files ending in `.markdown`. Each such file contains some frontmatter and some content written in the Markdown language. A sample content file to say "Hello World!" may look something like this:

```Markdown
---
layout: "page"
title: "Hello World!"
date: "2024/09/22"
---

# Hello World!

This is a content file to say "Hello World!".
```

The frontmatter of this file contains three variables, more, however, can be added. These variables are: `layout`, `title`, and `date`. The `layout` variable declares that this content file would like to use the layout called `page` located at the path `layouts/page.liquid` at the root of a Mandy project. The second and third variables are not strictly needed by Mandy, unless they are called upon in a layout via the `page.params` namespace. These three variables are called frontmatter and are enclosed in three dashes. Anything below the second set of three dashes is content written in Markdown.

## Routing

Routing in Mandy is controlled via the `baseurl` template variable. The value of this variable is set to either of the values stored in the `dev_url` or `prod_url` variables saved in the configuration file. Which one of these values is used, depends upon which variant the `MANDY_ENV` environment variable has been set to. This environment variable can be set to either `production` or `development`. Additionally, each `page` namespace has an `url` attribute. This attribute can be used to create links between different pages.

## Links

To develop your own Mandy project, you may want to visit the following links:

- [A guide to Markdown](https://www.markdownguide.org/)
- [A guide to Liquid](https://shopify.github.io/liquid/basics/introduction/)
- [A guide to SASS](https://www.youtube.com/watch?v=BEdCOvJ5RY4)
- [My personal website (a Mandy project)](https://github.com/alyxshang/alyxshang.github.io)