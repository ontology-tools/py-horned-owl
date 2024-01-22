{{ fullname | escape | underline}}

{% set defined_aliases = [ "ClassExpression","ObjectPropertyExpression","Literal","DataRange","Individual","PropertyExpression","AnnotationSubject","AnnotationValue","SubObjectPropertyExpression","Axiom" ] %}
{% set aliases = members | select("in", defined_aliases) %}


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
.. rubric:: Module aliases

{% for item in aliases %}
{%- if not item.startswith("_") %}
.. autodata:: {{ item }}
{%- endif -%}
{% endfor %}
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



