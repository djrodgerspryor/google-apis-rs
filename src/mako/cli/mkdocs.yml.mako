<%
    from util import (put_and, new_context)
    from cli import (subcommand_md_filename, mangle_subcommand)

    c = new_context(schemas, resources, context.get('methods'))

    def pretty(n):
        return ' '.join(s.capitalize() for s in mangle_subcommand(n).split('-'))
%>\
<%namespace name="util" file="../lib/util.mako"/>\
site_name: ${util.canonical_name()} v${util.crate_version()}
site_url: ${cargo.doc_base_url}/${util.crate_name()}
site_description: Write integrating applications with bcore

repo_url: ${util.github_source_root_url()}

docs_dir: ${mkdocs.docs_dir}
site_dir: ${mkdocs.site_dir}

pages:
- ['index.md', 'Home']
% for resource in sorted(c.rta_map.keys()):
% for method in sorted(c.rta_map[resource]):
- ['${subcommand_md_filename(resource, method)}', '${pretty(resource)}', '${pretty(method)}']
% endfor # each method
% endfor # each resource

theme: readthedocs

copyright: Copyright &copy; ${copyright.years}, ${put_and(["`%s`" % a for a in copyright.authors])}
