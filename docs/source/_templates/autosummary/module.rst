{{ fullname | escape | underline}}

{% set aliases = type_aliases.get(fullname, {}) %}


.. py:currentmodule:: {{ fullname }}
.. currentmodule:: {{ fullname }}
  
{% block attributes %}
{% if attributes %}
.. rubric:: Module Attributes

.. autosummary::
   :toctree:
   :nosignatures:
{% for item in attributes %}
   {{ item }}
{%- endfor %}
{% endif %}
{% endblock %}

{% block functions %}
{% if functions %}
.. rubric:: {{ _('Functions') }}

.. autosummary::
   :toctree:
   :nosignatures:
{% for item in functions %}
   {{ item }}
{%- endfor %}
{% endif %}
{% endblock %}

{% block classes %}
{% if classes %}
.. rubric:: {{ _('Classes') }}

.. autosummary::
   :toctree:
   :nosignatures:
{% for item in classes %}
   {{ item }}
{%- endfor %}
{% endif %}
{% endblock %}

{% block exceptions %}
{% if exceptions %}
.. rubric:: {{ _('Exceptions') }}

.. autosummary::
   :toctree:
   :nosignatures:
{% for item in exceptions %}
   {{ item }}
{%- endfor %}
{% endif %}
{% endblock %}

{% block aliases %}
{% if aliases %}
.. rubric:: Module aliases
{% for name, canonical in aliases | dictsort %}
.. py:type:: {{ name }}
   :canonical: {{ canonical }}
{% endfor %}
{% endif %}
{% endblock %}

{% block modules %}
{% if modules %}
.. rubric:: Modules

.. autosummary::
:toctree:
:recursive:
{% for item in modules %}
{{ item }}
{%- endfor %}
{% endif %}
{% endblock %}



